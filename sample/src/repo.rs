use domner_tech_sql_client::{
  CommandType, SqlRepo,
  pool_manager::{DbManager, DbRow, PooledClient},
};

use anyhow::Result;

pub struct UserRepo<'a> {
  db: &'a DbManager,
  conn_str: String,
}

impl<'a> UserRepo<'a> {
  pub fn new(conn_str: String, db: &'a DbManager) -> Self {
    Self { conn_str, db }
  }

  async fn get_client(&self) -> PooledClient {
    match self.db.get_client("mssql_pool").await {
      Ok(client) => client,
      Err(e) => panic!("Failed to get DB client: {}", e),
    }
  }

  pub async fn get_by_id(&mut self, id: i32) -> Result<Option<User>> {
    let mut client_pool = self.get_client().await;

    let user = SqlRepo::execute_command_single_query(
      &mut client_pool,
      "[dbo].[select_user]",
      &[&id],
      CommandType::StoreProcedure,
      |row| User::from_row(row),
    )
    .await?;
    Ok(user)
  }

  pub async fn get_users(&mut self) -> Result<Vec<User>> {
    let mut client_pool = self.get_client().await;

    let user = SqlRepo::execute_command_query(
      &mut client_pool,
      "select 1 as id, 'hi' as user_name, name, email, password from users;",
      &[],
      CommandType::Text,
      |row| User::from_row(row),
    )
    .await?;
    Ok(user)
  }
}

#[derive(Debug)]
pub struct User {
  pub id: i32,
  pub user_name: String,
  pub name: String,
  pub password: String,
  pub email: String,
}
impl User {
  pub fn from_row(row: &DbRow) -> Self {
    Self {
      id: row
        .get_mssql::<i32>("id")
        .expect("Failed to get id")
        .unwrap_or_default(), // fallback if null
      name: row
        .get_mssql::<&str>("name")
        .expect("Failed to get name")
        .unwrap_or_default()
        .to_string(),
      email: row
        .get_mssql::<&str>("email")
        .expect("Failed to get email")
        .unwrap_or_default()
        .to_string(),
      user_name: row
        .get_mssql::<&str>("user_name")
        .expect("Failed to get user_name")
        .unwrap_or_default()
        .to_string(),
      password: row
        .get_mssql::<&str>("password")
        .expect("Failed to get password")
        .unwrap_or_default()
        .to_string(),
    }
  }
}
