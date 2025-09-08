# `DomnerTech-SqlClient`

DomnerTech-SqlClient is for connect to SQL like MSSQL and PostgreSQL

## `📦 Unified SQL Repo`

A lightweight async SQL repository abstraction for MSSQL (Tiberius) and PostgreSQL (tokio-postgres).
It provides a unified API for executing queries, commands, and bulk inserts across different databases using feature flags.

## `✨ Features`

- ✅ Unified API for MSSQL and PostgreSQL
- ✅ Async connection pooling (custom pool manager)
- ✅ Unified parameter binding (UnifiedToSql)
- ✅ Query execution with mapping support
- ✅ Bulk insert with safe parameterization
- ✅ Feature-flagged: compile only the driver you need

## `🚀 Installation`

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
domner_tech_sql_client = { version = "0.1", features = ["mssql"] }
# or
domner_tech_sql_client = { version = "0.1", features = ["pgsql"] }
# or
domner_tech_sql_client = { version = "0.1", features = ["mssql, pgsql"] }
```

## Available features:

`mssql` → <b>Enables [tiberius](https://crates.io/crates/tiberius) for SQL Server</b>
`pgsql` → <b>Enables [tokio-postgres](https://crates.io/crates/tokio-postgres) for PostgreSQL</b>

## `⚡ Usage`

### Initialize a pool

```rs
use domner_tech_sql_client::pool_manager::DbManager;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manager = DbManager::new();

    // For PostgreSQL
    manager.init_pool(
        "pg_pool",
        "postgresql://postgres:admin@localhost:5432/mydb",
        5
    ).await?;

    // For MSSQL
    manager.init_pool(
        "mssql_pool",
        "server=tcp:localhost,1433;User Id=sa;Password=your_password;TrustServerCertificate=true;",
        5
    ).await?;

    Ok(())
}
```

### Execute a query

```rs
use domner_tech_sql_client::{SqlRepo, CommandType};
use domner_tech_sql_client::pool_manager::{DbRow, DbManager};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let manager = DbManager::new();
    manager.init_pool("pg_pool", "postgresql://postgres:admin@localhost:5432/mydb", 5).await?;

    let mut client = manager.get_client("pg_pool").await?;

    let rows = SqlRepo::execute_command_query(
        &mut client,
        "SELECT id, name FROM users WHERE id = $1",
        &[&1],
        CommandType::Text,
        |row: &DbRow| {
            #[cfg(feature = "pgsql")]
            {
                let id: i32 = row.get_pgsql(0).unwrap();
                let name: String = row.get_pgsql(1).unwrap();
                (id, name)
            }

            #[cfg(feature = "mssql")]
            {
                let id: Option<i32> = row.get_mssql("id").unwrap();
                let name: Option<String> = row.get_mssql("name").unwrap();
                (id, name)
            }
        }
    ).await?;

    println!("{:?}", rows);

    Ok(())
}
```

### Bulk insert

```rs
let entities: Vec<Vec<&dyn UnifiedToSql>> = vec![
    vec![&1, &"Alice"],
    vec![&2, &"Bob"],
];

let affected = SqlRepo::execute_bulk_insert(
    &mut client,
    "users",
    &["id", "name"],
    &entities.iter().map(|e| e.as_slice()).collect::<Vec<_>>(),
).await?;

```

## `🔧 Development`

- MSSQL support requires SQL Server running with TCP enabled.
- PostgreSQL support requires a `postgresql://` connection string.
- Run tests with the appropriate feature flag:

```bash
cargo test --features pgsql
cargo test --features mssql
```

## `📜 License`

Licensed under either of:

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
