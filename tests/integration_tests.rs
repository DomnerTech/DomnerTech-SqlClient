// use anyhow::Result;
// use domner_tech_sql_client::{
//   MssqlToSql, PgToSql,
//   pool_manager::{DbClient, DbManager},
// };

// // Use a unified ToSql for testing purposes
// // This is a common pattern to avoid having to use different types
// // in test functions.
// trait UnifiedToSql: MssqlToSql + PgToSql + Send + Sync {}
// impl<T> UnifiedToSql for T where T: MssqlToSql + PgToSql + Send + Sync {}

// // A simple utility to set up the DbManager for a test
// async fn setup_db_manager(conn_str: &str) -> Result<DbManager> {
//   let db_manager = DbManager::new();
//   db_manager.init_pool("test_pool", conn_str, 1).await?;
//   Ok(db_manager)
// }

// // --- MS SQL Tests ---
// #[cfg(feature = "mssql")]
// #[tokio::test]
// async fn mssql_test() -> Result<()> {
//   // Replace with your actual MS SQL connection string
//   let conn_str = "Server=localhost;Integrated Security=true";
//   let db_manager = setup_db_manager(conn_str).await?;
//   let mut client = db_manager.get_client("test_pool").await?;

//   // Example: Create a test table
//   let create_table_query = "CREATE TABLE TestTable (Id INT PRIMARY KEY, Name NVARCHAR(50))";
//   SqlRepo::execute_command_none_query(&mut client, create_table_query, &[], CommandType::Text)
//     .await?;

//   // Example: Insert a row
//   let name = "TestUser";
//   let insert_query = "INSERT INTO TestTable (Id, Name) VALUES (@P1, @P2)";
//   let rows_affected = SqlRepo::execute_command_none_query(
//     &mut client,
//     insert_query,
//     &[&1_i32 as &dyn MssqlToSql, &name as &dyn MssqlToSql],
//     CommandType::Text,
//   )
//   .await?;
//   assert_eq!(rows_affected, 1);

//   // Example: Query the inserted row
//   let select_query = "SELECT Id, Name FROM TestTable WHERE Id = @P1";
//   let result = SqlRepo::execute_command_single_query(
//     &mut client,
//     select_query,
//     &[&1_i32 as &dyn MssqlToSql],
//     CommandType::Text,
//     |row| {
//       let id: i32 = row.get(0);
//       let name: String = row.get(1);
//       (id, name)
//     },
//   )
//   .await?;
//   assert!(result.is_some());
//   let (id, name) = result.unwrap();
//   assert_eq!(id, 1);
//   assert_eq!(name, "TestUser");

//   // Clean up
//   let drop_table_query = "DROP TABLE TestTable";
//   SqlRepo::execute_command_none_query(&mut client, drop_table_query, &[], CommandType::Text)
//     .await?;

//   Ok(())
// }

// // --- PostgreSQL Tests ---
// #[cfg(feature = "pgsql")]
// #[tokio::test]
// async fn pgsql_test() -> Result<()> {
//   // Replace with your actual PostgreSQL connection string
//   let conn_str = "postgresql://user:password@localhost:5432/database";
//   let db_manager = setup_db_manager(conn_str).await?;
//   let mut client = db_manager.get_client("test_pool").await?;

//   // Example: Create a test table
//   let create_table_query =
//     "CREATE TABLE IF NOT EXISTS test_table (id INT PRIMARY KEY, name VARCHAR(50))";
//   SqlRepo::execute_command_none_query(&mut client, create_table_query, &[], CommandType::Text)
//     .await?;

//   // Example: Insert a row
//   let name = "TestUser";
//   let insert_query = "INSERT INTO test_table (id, name) VALUES ($1, $2)";
//   let rows_affected = SqlRepo::execute_command_none_query(
//     &mut client,
//     insert_query,
//     &[&1_i32 as &dyn PgToSql, &name as &dyn PgToSql],
//     CommandType::Text,
//   )
//   .await?;
//   assert_eq!(rows_affected, 1);

//   // Example: Query the inserted row
//   let select_query = "SELECT id, name FROM test_table WHERE id = $1";
//   let result = SqlRepo::execute_command_single_query(
//     &mut client,
//     select_query,
//     &[&1_i32 as &dyn PgToSql],
//     CommandType::Text,
//     |row| {
//       let id: i32 = row.get(0);
//       let name: String = row.get(1);
//       (id, name)
//     },
//   )
//   .await?;
//   assert!(result.is_some());
//   let (id, name) = result.unwrap();
//   assert_eq!(id, 1);
//   assert_eq!(name, "TestUser");

//   // Clean up
//   let drop_table_query = "DROP TABLE test_table";
//   SqlRepo::execute_command_none_query(&mut client, drop_table_query, &[], CommandType::Text)
//     .await?;

//   Ok(())
// }
