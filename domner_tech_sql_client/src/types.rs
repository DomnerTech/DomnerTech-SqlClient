use anyhow::Result;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
#[cfg(feature = "mssql")]
use tiberius::ToSql as MssqlToSql;
#[cfg(feature = "pgsql")]
use tokio_postgres::types::ToSql as PgToSql;
use uuid::Uuid;

pub trait UnifiedToSql {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql>;
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)>;
}

impl UnifiedToSql for i16 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<i16> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
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

impl UnifiedToSql for Option<i32> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for i64 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<i64> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for f32 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<f32> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for f64 {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<f64> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Vec<u8> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<Vec<u8>> {
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

impl UnifiedToSql for Option<&str> {
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

impl UnifiedToSql for Option<String> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Uuid {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<Uuid> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for bool {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<bool> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for NaiveDate {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<NaiveDate> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for NaiveTime {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<NaiveTime> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for DateTime<Utc> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<DateTime<Utc>> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for NaiveDateTime {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}

impl UnifiedToSql for Option<NaiveDateTime> {
  #[cfg(feature = "mssql")]
  fn to_mssql_param(&self) -> Result<&dyn MssqlToSql> {
    Ok(self)
  }
  #[cfg(feature = "pgsql")]
  fn to_pgsql_param(&self) -> Result<&(dyn PgToSql + Sync)> {
    Ok(self)
  }
}
