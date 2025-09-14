mod repo;

use crate::repo::UserRepo;
use anyhow::Result;
use chrono::prelude::*;
use domner_tech_sql_client::pool_manager::DbManager;

#[tokio::main]
async fn main() -> Result<()> {
  println!("Starting database connection test...");

  println!("\nAttempting to connect to MS SQL...");
  // Replace with your actual MS SQL connection string
  let conn_str_mssql = "Data Source=DESKTOP-M076BMC; Initial Catalog=crud-rust; User Id=sa; Password=admin123; Trusted_Connection=Yes; TrustServerCertificate=True; Integrated Security=false";
  let db_manager = DbManager::new();

  let mut repo = UserRepo::new(&db_manager);
  const MSSQL_POOL: &str = "mssql_pool";
  if let Err(e) = db_manager.init_pool(MSSQL_POOL, conn_str_mssql, 1).await {
    eprintln!("Failed to connect to MS SQL: {}", e);
  } else {
    println!("MS SQL connection pool initialized successfully.");
    if let Some(user) = repo.get_by_id(MSSQL_POOL, 2).await? {
      println!("{:?}", user);
    } else {
      println!("Failed to get user.")
    }
  }

  println!("\nAttempting to connect to PostgreSQL...");
  // Replace with your actual PostgreSQL connection string
  const PGSQL_POOL: &str = "pgsql_pool";
  let conn_str_pgsql = "postgresql://postgres:admin@localhost:5432/crud-rust";
  if let Err(e) = db_manager.init_pool(PGSQL_POOL, conn_str_pgsql, 1).await {
    eprintln!("Failed to connect to PostgreSQL: {}", e);
  } else {
    println!("PostgreSQL connection pool initialized successfully.");
    if let Ok(users) = repo.get_users(PGSQL_POOL, Utc::now()).await {
      println!("{:?}", users);
    } else {
      println!("Failed to get users.")
    }
  }
  println!("\nTesting complete.");

  Ok(())
}
