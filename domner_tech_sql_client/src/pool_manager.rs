use anyhow::Result;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[cfg(feature = "mssql")]
mod mssql_ops {
  pub use crate::MssqlRow;
  pub use tiberius::{Client, Config, FromSql};
  pub use tokio::net::TcpStream;
  pub use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};
  /// Helper function to wrap a TcpStream for Tiberius
  pub fn compat_stream(stream: TcpStream) -> Compat<TcpStream> {
    stream.compat_write()
  }
}

#[cfg(feature = "pgsql")]
mod pgsql_ops {
  pub use crate::PgRow;
  pub use tokio_postgres::types::FromSql;
  pub use tokio_postgres::{Client as PgClient, NoTls, connect};
}

// The DbClient type will be an enum that is only compiled if the corresponding feature is enabled.
pub enum DbClient {
  #[cfg(feature = "mssql")]
  Mssql(mssql_ops::Client<mssql_ops::Compat<mssql_ops::TcpStream>>),
  #[cfg(feature = "pgsql")]
  Pgsql(pgsql_ops::PgClient),
}

#[derive(Debug, Clone, Copy)]
pub enum DbClientType {
  #[cfg(feature = "mssql")]
  Mssql,
  #[cfg(feature = "pgsql")]
  Pgsql,
}

pub type DbPool = Arc<Mutex<Vec<DbClient>>>;

pub struct DbManager {
  pools: Arc<Mutex<HashMap<String, DbPool>>>,
}

impl DbManager {
  pub fn new() -> Self {
    Self {
      pools: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  /// Initialize a connection pool for a given name (if not exists)
  pub async fn init_pool(&self, pool_name: &str, conn_str: &str, pool_size: u32) -> Result<()> {
    let mut pools = self.pools.lock().await;
    if pools.contains_key(pool_name) {
      return Ok(());
    }

    let mut connections = Vec::with_capacity(pool_size as usize);

    if conn_str.starts_with("postgresql://") || conn_str.starts_with("postgres://") {
      #[cfg(feature = "pgsql")]
      for _ in 0..pool_size {
        let (client, connection) = pgsql_ops::connect(
          &DbManager::postgres_url_to_tokio(conn_str),
          pgsql_ops::NoTls,
        )
        .await?;
        tokio::spawn(async move {
          if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
          }
        });
        connections.push(DbClient::Pgsql(client));
      }
    } else {
      #[cfg(feature = "mssql")]
      if let Some(config) = mssql_ops::Config::from_ado_string(conn_str)
        .ok()
        .or_else(|| mssql_ops::Config::from_jdbc_string(conn_str).ok())
      {
        for _ in 0..pool_size {
          let mut config = config.clone();
          config.trust_cert();
          let tcp = mssql_ops::TcpStream::connect(config.get_addr()).await?;
          tcp.set_nodelay(true)?;
          let client = mssql_ops::Client::connect(config, mssql_ops::compat_stream(tcp)).await?;
          connections.push(DbClient::Mssql(client));
        }
      }
    }

    pools.insert(pool_name.to_string(), Arc::new(Mutex::new(connections)));
    Ok(())
  }

  /// Get a pooled client wrapped in a guard (auto-return when dropped)
  pub async fn get_client(&self, pool_name: &str) -> Result<PooledClient> {
    let pools = self.pools.lock().await;
    let pool = pools
      .get(pool_name)
      .ok_or_else(|| anyhow::anyhow!("Pool `{}` not found", pool_name))?;
    let mut pool_guard = pool.lock().await;
    let client = pool_guard
      .pop()
      .ok_or_else(|| anyhow::anyhow!("Pool `{}` is empty", pool_name))?;
    Ok(PooledClient {
      name: pool_name.to_string(),
      client: Some(client),
      manager: self.clone(),
    })
  }

  /// Return a client back to the pool
  async fn return_client(&self, pool_name: &str, client: DbClient) -> Result<()> {
    let pools = self.pools.lock().await;
    let pool = pools
      .get(pool_name)
      .ok_or_else(|| anyhow::anyhow!("Pool `{}` not found", pool_name))?;
    let mut pool_guard = pool.lock().await;
    pool_guard.push(client);
    Ok(())
  }

  fn postgres_url_to_tokio(conn_url: &str) -> String {
    // Parse the URL
    // Remove the prefix
    let conn_str = conn_url
      .strip_prefix("postgresql://")
      .expect("Invalid connection string");

    // Split by '@' to separate userinfo and hostinfo
    let parts: Vec<&str> = conn_str.split('@').collect();
    if parts.len() != 2 {
      panic!("Invalid connection string format");
    }

    // Parse username and password
    let user_pass: Vec<&str> = parts[0].split(':').collect();
    if user_pass.len() != 2 {
      panic!("Invalid username/password format");
    }
    let username = user_pass[0];
    let password = user_pass[1];

    // Parse host, port, and dbname
    let host_port_db: Vec<&str> = parts[1].split('/').collect();
    if host_port_db.len() != 2 {
      panic!("Invalid host/db format");
    }

    let host_port: Vec<&str> = host_port_db[0].split(':').collect();
    if host_port.len() != 2 {
      panic!("Invalid host/port format");
    }
    let host = host_port[0];
    let port = host_port[1];

    // Extract dbname before '?' if query params exist
    let dbname = host_port_db[1]
      .split('?')
      .next()
      .expect("Invalid dbname format");

    format!(
      "host={} user={} password={} port={} dbname={}",
      host, username, password, port, dbname
    )
  }
}

impl Clone for DbManager {
  fn clone(&self) -> Self {
    Self {
      pools: self.pools.clone(),
    }
  }
}

/// Wrapper that auto-returns client when dropped
pub struct PooledClient {
  pub name: String,
  pub client: Option<DbClient>,
  pub manager: DbManager,
}
impl PooledClient {
  pub fn client(&mut self) -> &mut DbClient {
    self.client.as_mut().unwrap()
  }
  pub fn client_ref(&self) -> &DbClient {
    self.client.as_ref().unwrap()
  }
}
impl Drop for PooledClient {
  fn drop(&mut self) {
    if let Some(client) = self.client.take() {
      let name = self.name.clone();
      let manager = self.manager.clone();
      // Return client asynchronously in background
      tokio::spawn(async move {
        let _ = manager.return_client(&name, client).await;
      });
    }
  }
}

pub enum DbRow<'a> {
  #[cfg(feature = "mssql")]
  Mssql(&'a mssql_ops::MssqlRow),
  #[cfg(feature = "pgsql")]
  Pgsql(&'a pgsql_ops::PgRow),
}

impl<'a> DbRow<'a> {
  #[cfg(feature = "mssql")]
  pub fn get_mssql<'r, R>(&self, idx: &str) -> Result<R>
  where
    'a: 'r,
    R: mssql_ops::FromSql<'r>,
  {
    match self {
      DbRow::Mssql(row) => {
        let value: Result<Option<R>> = row.try_get::<R, &str>(idx).map_err(|e| anyhow::anyhow!(e));
        if let Ok(Some(val)) = value {
          return Ok(val);
        }
        Err(anyhow::anyhow!("Failed to get {}", idx))
      }
      _ => Err(anyhow::anyhow!("Mismatched database driver")),
    }
  }

  #[cfg(feature = "pgsql")]
  pub fn get_pgsql<'p, T>(&'p self, idx: &str) -> Result<T>
  where
    T: pgsql_ops::FromSql<'p>,
  {
    match self {
      DbRow::Pgsql(row) => {
        let value = row.try_get::<&str, T>(idx).map_err(|e| anyhow::anyhow!(e));
        value
      }
      _ => Err(anyhow::anyhow!("Mismatched database driver")),
    }
  }
}
