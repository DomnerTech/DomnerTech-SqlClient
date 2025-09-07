mod repo;

use anyhow::Result;
use domner_tech_sql_client::pool_manager::DbManager;

use crate::repo::UserRepo;

#[tokio::main]
async fn main() -> Result<()> {
  println!("Starting database connection test...");

  println!("\nAttempting to connect to MS SQL...");
  // Replace with your actual MS SQL connection string
  let conn_str_mssql = "Data Source=DESKTOP-M076BMC; Initial Catalog=crud-rust; User Id=sa; Password=admin123; Trusted_Connection=Yes; TrustServerCertificate=True; Integrated Security=false";
  let db_manager = DbManager::new();

  if let Err(e) = db_manager.init_pool("mssql_pool", conn_str_mssql, 1).await {
    eprintln!("Failed to connect to MS SQL: {}", e);
  } else {
    println!("MS SQL connection pool initialized successfully.");
    let mut repo = UserRepo::new(conn_str_mssql.into(), &db_manager);
    if let Some(user) = repo.get_by_id(2).await? {
      println!("{:?}", user);
    } else {
      println!("Failed to get user.")
    }
  }

  println!("\nAttempting to connect to PostgreSQL...");
  // Replace with your actual PostgreSQL connection string
  let conn_str_pgsql = "postgresql://postgres:admin@localhost:5432/crud-rust?schema=public";
  if let Err(e) = db_manager.init_pool("pgsql_pool", conn_str_pgsql, 1).await {
    eprintln!("Failed to connect to PostgreSQL: {}", e);
  } else {
    println!("PostgreSQL connection pool initialized successfully.");
    let mut repo = UserRepo::new(conn_str_pgsql.into(), &db_manager);
    if let Ok(users) = repo.get_users().await {
      println!("{:?}", users);
    } else {
      println!("Failed to get users.")
    }
  }
  println!("\nTesting complete.");

  Ok(())
}
