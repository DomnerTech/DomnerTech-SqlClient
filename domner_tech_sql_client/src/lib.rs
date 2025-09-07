pub mod pool_manager;

use anyhow::Result;

#[cfg(feature = "mssql")]
pub use tiberius::Row as MssqlRow;

#[cfg(feature = "mssql")]
pub use tiberius::ToSql as MssqlToSql;

#[cfg(feature = "pgsql")]
pub use tokio_postgres::types::ToSql as PgToSql;

#[cfg(feature = "pgsql")]
pub use tokio_postgres::Row as PgRow;

use crate::pool_manager::{DbClient, DbRow, PooledClient};

#[derive(Debug, Clone, Copy)]
pub enum CommandType {
  Text,
  StoreProcedure,
  TableDirect,
}
impl CommandType {
  fn prefix(&self) -> &'static str {
    match self {
      CommandType::Text => "",
      CommandType::StoreProcedure => "EXEC ",
      CommandType::TableDirect => "SELECT * FROM ",
    }
  }
}

pub trait UnifiedToSql {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql>;
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)>;
}

impl UnifiedToSql for i32 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for &str {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for String {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

pub struct SqlRepo;

impl SqlRepo {
  fn build_query_with_params(cmd_txt: &str, cmd_type: CommandType, params_count: usize) -> String {
    match cmd_type {
      CommandType::Text => cmd_txt.to_string(),
      CommandType::StoreProcedure => {
        let placeholders: Vec<String> = (0..params_count).map(|i| format!("@P{}", i + 1)).collect();

        if placeholders.is_empty() {
          format!("{}{}", cmd_type.prefix(), cmd_txt)
        } else {
          format!(
            "{}{} {}",
            cmd_type.prefix(),
            cmd_txt,
            placeholders.join(", ")
          )
        }
      }
      CommandType::TableDirect => format!("{}{}", cmd_type.prefix(), cmd_txt),
    }
  }

  pub async fn execute_command_none_query(
    pooled_client: &mut PooledClient,
    cmd_txt: &str,
    params: &[&dyn UnifiedToSql],
    cmd_type: CommandType,
  ) -> Result<u64> {
    let query = Self::build_query_with_params(cmd_txt, cmd_type, params.len());
    let client = pooled_client.client();
    let result = match client {
      //client.execute(&query, &params).await?;
      #[cfg(feature = "mssql")]
      DbClient::Mssql(c) => {
        let mssql_params: Result<Vec<&dyn MssqlToSql>> =
          params.iter().map(|p| p.to_mssql_param()).collect();
        Ok(c.execute(&query, mssql_params?.as_slice()).await?.total())
      }
      #[cfg(feature = "pgsql")]
      DbClient::Pgsql(c) => {
        let pg_params: Result<Vec<&(dyn PgToSql + Sync)>> =
          params.iter().map(|p| p.to_pgsql_param()).collect();
        Ok(c.execute(&query, pg_params?.as_slice()).await?)
      }
      #[cfg(not(any(feature = "mssql", feature = "pgsql")))]
      _ => return Err(anyhow::anyhow!("No database feature enabled.")),
    };
    result
  }

  pub async fn execute_bulk_insert(
    pooled_client: &mut PooledClient,
    table: &str,
    columns: &[&str],
    entities: &[&[&dyn UnifiedToSql]],
  ) -> Result<u64> {
    let mut values = Vec::new();
    for entity in entities {
      let value_placeholders: Vec<String> =
        (0..entity.len()).map(|i| format!("@P{}", i + 1)).collect();
      values.push(format!("({})", value_placeholders.join(", ")));
    }

    let client = pooled_client.client();

    let query = format!(
      "INSERT INTO {} ({}) VALUES {}",
      table,
      columns.join(", "),
      values.join(", ")
    );
    let result = match client {
      #[cfg(feature = "mssql")]
      DbClient::Mssql(c) => Ok(c.execute(&query, &[]).await?.total()),
      #[cfg(feature = "pgsql")]
      DbClient::Pgsql(c) => Ok(c.execute(&query, &[]).await?),
      #[cfg(not(any(feature = "mssql", feature = "pgsql")))]
      _ => return Err(anyhow::anyhow!("No database feature enabled.")),
    };
    result
  }

  pub async fn execute_command_query<T>(
    pooled_client: &mut PooledClient,
    cmd_txt: &str,
    params: &[&dyn UnifiedToSql],
    cmd_type: CommandType,
    map_rows: impl Fn(&DbRow) -> T,
  ) -> Result<Vec<T>> {
    if cmd_txt.trim().is_empty() {
      return Ok(Vec::new());
    }

    let query = Self::build_query_with_params(cmd_txt, cmd_type, params.len());
    let client = pooled_client.client();

    let db_rows = match client {
      #[cfg(feature = "mssql")]
      DbClient::Mssql(c) => {
        let mssql_params: Result<Vec<&dyn MssqlToSql>> =
          params.iter().map(|p| p.to_mssql_param()).collect();
        let stream = c.query(query, mssql_params?.as_slice()).await?;
        let rows = stream.into_results().await?;
        let mut results: Vec<T> = Vec::new();
        for row in &rows[0] {
          results.push(map_rows(&DbRow::Mssql(row)));
        }
        results
      }

      #[cfg(feature = "pgsql")]
      DbClient::Pgsql(c) => {
        let pg_params: Result<Vec<&(dyn PgToSql + Sync)>> =
          params.iter().map(|p| p.to_pgsql_param()).collect();
        let rows = c.query(&query, pg_params?.as_slice()).await?;

        let mut results: Vec<T> = Vec::new();
        for row in &rows {
          results.push(map_rows(&DbRow::Pgsql(&row.clone())));
        }
        results
      }

      #[cfg(not(any(feature = "mssql", feature = "pgsql")))]
      _ => return Err(anyhow::anyhow!("No database feature enabled.")),
    };
    Ok(db_rows)
  }

  /// Execute a query returning a single row
  /// T is a closure that maps a Row to the desired type
  /// Returns None if no rows found
  pub async fn execute_command_single_query<T>(
    pooled_client: &mut PooledClient,
    cmd_txt: &str,
    params: &[&dyn UnifiedToSql],
    cmd_type: CommandType,
    map_row: impl Fn(&DbRow) -> T,
  ) -> Result<Option<T>> {
    let mut rows =
      Self::execute_command_query(pooled_client, cmd_txt, params, cmd_type, map_row).await?;
    Ok(rows.pop())
  }
}
