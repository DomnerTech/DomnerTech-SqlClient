pub mod pool_manager;
pub mod types;

use crate::pool_manager::DbClientType;
use crate::pool_manager::{DbClient, DbRow, PooledClient};
use crate::types::UnifiedToSql;

#[cfg(feature = "mssql")]
use crate::types::sql::mssql;

#[cfg(feature = "pgsql")]
use crate::types::sql::pgsql;

pub use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum CommandType {
  Text,
  StoreProcedure,
  TableDirect,
  #[cfg(feature = "pgsql")]
  Function,
}
impl CommandType {
  fn prefix(&self, db_type: Option<&DbClientType>) -> &'static str {
    match self {
      CommandType::Text => "",
      CommandType::StoreProcedure => match db_type.unwrap() {
        #[cfg(feature = "mssql")]
        DbClientType::Mssql => "EXEC ",
        #[cfg(feature = "pgsql")]
        DbClientType::Pgsql => "CALL ",
        #[cfg(not(any(feature = "mssql", feature = "pgsql")))]
        _ => panic!("No database feature enabled."),
      },
      #[cfg(feature = "pgsql")]
      CommandType::Function => "SELECT * FROM ",
      CommandType::TableDirect => "SELECT * FROM ",
    }
  }
}

pub struct SqlRepo;

impl SqlRepo {
  fn build_query_with_params(
    db_type: DbClientType,
    cmd_txt: &str,
    cmd_type: CommandType,
    params_count: usize,
  ) -> String {
    match cmd_type {
      CommandType::Text => cmd_txt.to_string(),
      CommandType::StoreProcedure => match db_type {
        #[cfg(feature = "mssql")]
        DbClientType::Mssql => {
          let placeholders: Vec<String> =
            (0..params_count).map(|i| format!("@P{}", i + 1)).collect();
          if placeholders.is_empty() {
            format!("{}{}", cmd_type.prefix(Some(&db_type)), cmd_txt)
          } else {
            format!(
              "{}{} {}",
              cmd_type.prefix(Some(&db_type)),
              cmd_txt,
              placeholders.join(", ")
            )
          }
        }
        #[cfg(feature = "pgsql")]
        DbClientType::Pgsql => {
          let placeholders: Vec<String> = (1..=params_count).map(|i| format!("${}", i)).collect();
          if placeholders.is_empty() {
            format!("{}{}()", cmd_type.prefix(Some(&db_type)), cmd_txt)
          } else {
            format!(
              "{}{}({})",
              cmd_type.prefix(Some(&db_type)),
              cmd_txt,
              placeholders.join(", ")
            )
          }
        }
        #[cfg(not(any(feature = "mssql", feature = "pgsql")))]
        _ => panic!("No database feature enabled."),
      },
      CommandType::TableDirect => format!("{}{}", cmd_type.prefix(None), cmd_txt),
      #[cfg(feature = "pgsql")]
      CommandType::Function => {
        let placeholders: Vec<String> = (1..=params_count).map(|i| format!("${}", i)).collect();
        if placeholders.is_empty() {
          format!("{}{}()", cmd_type.prefix(None), cmd_txt)
        } else {
          format!(
            "{}{}({})",
            cmd_type.prefix(None),
            cmd_txt,
            placeholders.join(", ")
          )
        }
      }
    }
  }

  pub async fn execute_command_none_query(
    pooled_client: &mut PooledClient,
    cmd_txt: &str,
    params: &[&dyn UnifiedToSql],
    cmd_type: CommandType,
  ) -> Result<u64> {
    let client = pooled_client.client();
    match client {
      //client.execute(&query, &params).await?;
      #[cfg(feature = "mssql")]
      DbClient::Mssql(c) => {
        let mssql_params: Result<Vec<&dyn mssql::ToSql>> =
          params.iter().map(|p| p.to_mssql_param()).collect();
        let query =
          Self::build_query_with_params(DbClientType::Mssql, cmd_txt, cmd_type, params.len());
        Ok(c.execute(&query, mssql_params?.as_slice()).await?.total())
      }
      #[cfg(feature = "pgsql")]
      DbClient::Pgsql(c) => {
        let pg_params: Result<Vec<&(dyn pgsql::types::ToSql + Sync)>> =
          params.iter().map(|p| p.to_pgsql_param()).collect();
        let query =
          Self::build_query_with_params(DbClientType::Pgsql, cmd_txt, cmd_type, params.len());
        Ok(c.execute(&query, pg_params?.as_slice()).await?)
      }
      #[cfg(not(any(feature = "mssql", feature = "pgsql")))]
      _ => return Err(anyhow::anyhow!("No database feature enabled.")),
    }
  }

  pub async fn execute_bulk_insert(
    pooled_client: &mut PooledClient,
    table: &str,
    columns: &[&str],
    entities: &[&[&dyn UnifiedToSql]],
  ) -> Result<u64> {
    if entities.is_empty() {
      return Ok(0);
    }

    let client = pooled_client.client();

    match client {
      #[cfg(feature = "mssql")]
      DbClient::Mssql(c) => {
        let mut values = Vec::new();
        let mut flat_params: Vec<&dyn mssql::ToSql> = Vec::new();

        for (row_idx, entity) in entities.iter().enumerate() {
          let mut row_placeholders = Vec::new();
          for (col_idx, param) in entity.iter().enumerate() {
            let param_index = row_idx * entity.len() + col_idx + 1;
            row_placeholders.push(format!("@P{}", param_index));
            flat_params.push(param.to_mssql_param()?);
          }
          values.push(format!("({})", row_placeholders.join(", ")));
        }

        let query = format!(
          "INSERT INTO {} ({}) VALUES {}",
          table,
          columns.join(", "),
          values.join(", ")
        );

        return Ok(c.execute(&query, &flat_params).await?.total());
      }
      #[cfg(feature = "pgsql")]
      DbClient::Pgsql(c) => {
        let mut values = Vec::new();
        let mut flat_params: Vec<&(dyn pgsql::types::ToSql + Sync)> = Vec::new();

        for (row_idx, entity) in entities.iter().enumerate() {
          let mut row_placeholders = Vec::new();
          for (col_idx, param) in entity.iter().enumerate() {
            let param_index = row_idx * entity.len() + col_idx + 1;
            row_placeholders.push(format!("${}", param_index));
            flat_params.push(param.to_pgsql_param()?);
          }
          values.push(format!("({})", row_placeholders.join(", ")));
        }

        let query = format!(
          "INSERT INTO {} ({}) VALUES {}",
          table,
          columns.join(", "),
          values.join(", ")
        );

        return Ok(c.execute(&query, &flat_params).await?);
      }
      #[cfg(not(any(feature = "mssql", feature = "pgsql")))]
      _ => Err(anyhow::anyhow!("No database feature enabled.")),
    }
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

    let client = pooled_client.client();

    let db_rows = match client {
      #[cfg(feature = "mssql")]
      DbClient::Mssql(c) => {
        use crate::pool_manager::DbClientType;

        let mssql_params: Result<Vec<&dyn mssql::ToSql>> =
          params.iter().map(|p| p.to_mssql_param()).collect();
        let query =
          Self::build_query_with_params(DbClientType::Mssql, cmd_txt, cmd_type, params.len());
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
        let pg_params: Result<Vec<&(dyn pgsql::types::ToSql + Sync)>> =
          params.iter().map(|p| p.to_pgsql_param()).collect();
        let query =
          Self::build_query_with_params(DbClientType::Pgsql, cmd_txt, cmd_type, params.len());
        let rows = c.query(&query, pg_params?.as_slice()).await?;
        println!("{} {}", query, rows.len());
        let mut results: Vec<T> = Vec::new();
        for row in &rows {
          results.push(map_rows(&DbRow::Pgsql(row)));
        }
        results
      }
      #[cfg(not(any(feature = "mssql", feature = "pgsql")))]
      _ => return Err(anyhow::anyhow!("No database feature enabled.")),
    };
    Ok(db_rows)
  }

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
