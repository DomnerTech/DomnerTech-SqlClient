pub mod decimal;
pub mod sql;
pub mod uuid;

#[cfg(feature = "mssql")]
use crate::types::sql::mssql;

#[cfg(feature = "pgsql")]
use crate::types::sql::pgsql;

pub use anyhow::Result;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde_json::Value;
pub trait UnifiedToSql {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql>;
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)>;
}

impl UnifiedToSql for i16 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<i16> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for i32 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<i32> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for i64 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<i64> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for f32 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<f32> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for f64 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<f64> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Vec<u8> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<Vec<u8>> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for &str {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<&str> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for String {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<String> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for uuid::Uuid {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<uuid::Uuid> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for bool {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<bool> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for NaiveDate {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<NaiveDate> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for NaiveTime {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<NaiveTime> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for DateTime<Utc> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<DateTime<Utc>> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for NaiveDateTime {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<NaiveDateTime> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Value {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn mssql::ToSql> {
    todo!("Not implemented for MSSQL yet");
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn pgsql::types::ToSql + Sync)> {
    Ok(self)
  }
}
