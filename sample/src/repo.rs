use chrono::{DateTime, NaiveDateTime, Utc};

use anyhow::Result;
use domner_tech_sql_client::{
  CommandType, SqlRepo,
  pool_manager::{DbManager, DbRow, PooledClient},
  types::decimal::decimal::Decimal,
};

pub struct UserRepo<'a> {
  db: &'a DbManager,
}

impl<'a> UserRepo<'a> {
  pub fn new(db: &'a DbManager) -> Self {
    Self { db }
  }

  async fn get_client(&self, pool_name: &str) -> PooledClient {
    match self.db.get_client(pool_name).await {
      Ok(client) => client,
      Err(e) => panic!("Failed to get DB client: {}", e),
    }
  }

  pub async fn get_by_id(&mut self, pool_name: &str, id: i32) -> Result<Option<User>> {
    let mut client_pool = self.get_client(pool_name).await;

    let user = SqlRepo::execute_command_single_query(
      &mut client_pool,
      "[dbo].[select_user]",
      &[&id],
      CommandType::StoreProcedure,
      |row| User::from(row),
    )
    .await?;
    Ok(user)
  }

  pub async fn get_users(&mut self, pool_name: &str, datetime: DateTime<Utc>) -> Result<Vec<User>> {
    let mut client_pool = self.get_client(pool_name).await;
    let user = SqlRepo::execute_command_query(
      &mut client_pool,
      "public.get_users",
      &[&datetime],
      CommandType::Function,
      |row| User::from(row),
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
  pub price: Decimal,
  pub created_at: Option<DateTime<Utc>>,
}

impl From<&DbRow<'_>> for User {
  fn from(value: &DbRow) -> Self {
    match value {
      DbRow::Mssql(row) => {
        let naive_created_at: NaiveDateTime = row
          .try_get::<NaiveDateTime, &str>("created_at")
          .expect("Failed to get created_at")
          .unwrap_or_default();
        let created_at: DateTime<Utc> =
          DateTime::<Utc>::from_naive_utc_and_offset(naive_created_at, Utc);
        Self {
          id: row
            .try_get::<i32, &str>("id")
            .expect("Failed to get id")
            .unwrap_or_default(), // fallback if null
          name: row
            .try_get::<&str, &str>("name")
            .expect("Failed to get name")
            .unwrap_or_default()
            .to_string(),
          email: row
            .try_get::<&str, &str>("email")
            .expect("Failed to get email")
            .unwrap_or_default()
            .to_string(),
          user_name: row
            .try_get::<&str, &str>("user_name")
            .expect("Failed to get user_name")
            .unwrap_or_default()
            .to_string(),
          password: row
            .try_get::<&str, &str>("password")
            .expect("Failed to get password")
            .unwrap_or_default()
            .to_string(),
          created_at: Some(created_at),
          price: row
            .try_get::<Decimal, &str>("price")
            .expect("Failed to get price")
            .unwrap_or_default(),
        }
      }
      DbRow::Pgsql(row) => {
        let naive_created_at: NaiveDateTime = value
          .get_pgsql::<NaiveDateTime>("created_at")
          .unwrap_or_default();

        let created_at: DateTime<Utc> =
          DateTime::<Utc>::from_naive_utc_and_offset(naive_created_at, Utc);
        Self {
          id: row.try_get::<&str, i32>("id").unwrap_or_default(), // fallback if null
          name: row
            .try_get::<&str, &str>("name")
            .unwrap_or_default()
            .to_string(),
          email: row
            .try_get::<&str, &str>("email")
            .unwrap_or_default()
            .to_string(),
          user_name: row
            .try_get::<&str, &str>("user_name")
            .unwrap_or_default()
            .to_string(),
          password: value
            .get_pgsql::<&str>("password")
            .unwrap_or_default()
            .to_string(),
          created_at: Some(created_at),
          price: row.try_get::<&str, Decimal>("price").unwrap_or_default(),
        }
      }
    }
  }
}
