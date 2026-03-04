use diesel_migrations::{EmbeddedMigrations, embed_migrations};

pub const PG_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./tests/data_pg/migrations");
mod data_pg;

#[tokio::test]
#[cfg(all(feature = "async", feature = "postgres"))]
async fn async_switch_schema_pg() {
    use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
    use diesel_async::{AsyncConnection, RunQueryDsl};
    use diesel_migrations::MigrationHarness;
    use diesel_schemas::async_conn::AsyncSchemaConnection;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = diesel_async::AsyncPgConnection::establish(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations via AsyncConnectionWrapper in a blocking task
    let url = database_url.clone();
    tokio::task::spawn_blocking(move || {
        let mut sync_conn =
            <AsyncConnectionWrapper<diesel_async::AsyncPgConnection> as diesel::Connection>::establish(&url)
                .expect("Failed to establish sync wrapper connection");
        sync_conn
            .run_pending_migrations(PG_MIGRATIONS)
            .expect("Failed to run migrations");
    })
    .await
    .expect("Blocking task panicked");

    // Get all books and pages in the default schema
    use data_pg::schema::{books, pages};
    let default_books = books::table
        .load::<(i32, String)>(&mut connection)
        .await
        .expect("Failed to load books from default schema");
    assert_eq!(
        default_books.len(),
        0,
        "Expected no books in default schema"
    );
    let default_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .await
        .expect("Failed to load pages from default schema");
    assert_eq!(
        default_pages.len(),
        0,
        "Expected no pages in default schema"
    );

    // Switch to a new schema
    connection
        .set_schema("async_new_schema")
        .await
        .expect("Failed to switch schema");

    // Run migrations in the new schema via AsyncConnectionWrapper
    let url = database_url.clone();
    tokio::task::spawn_blocking(move || {
        use diesel::connection::SimpleConnection;
        let mut sync_conn =
            <AsyncConnectionWrapper<diesel_async::AsyncPgConnection> as diesel::Connection>::establish(&url)
                .expect("Failed to establish sync wrapper connection");
        sync_conn
            .batch_execute("CREATE SCHEMA IF NOT EXISTS async_new_schema")
            .expect("Failed to create schema");
        sync_conn
            .batch_execute("SET SEARCH_PATH TO async_new_schema")
            .expect("Failed to switch schema");
        sync_conn
            .run_pending_migrations(PG_MIGRATIONS)
            .expect("Failed to run migrations in new schema");
    })
    .await
    .expect("Blocking task panicked");

    // Get all books and pages in the new schema (should be empty)
    let new_books = books::table
        .load::<(i32, String)>(&mut connection)
        .await
        .expect("Failed to load books from new schema");
    assert_eq!(new_books.len(), 0, "Expected no books in new schema");
    let new_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .await
        .expect("Failed to load pages from new schema");
    assert_eq!(new_pages.len(), 0, "Expected no pages in new schema");
}

pub const MYSQL_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./tests/data_mysql/migrations");
mod data_mysql;

#[tokio::test]
#[cfg(all(feature = "async", feature = "mysql"))]
async fn async_switch_schema_mysql() {
    use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
    use diesel_async::{AsyncConnection, RunQueryDsl};
    use diesel_migrations::MigrationHarness;
    use diesel_schemas::async_conn::AsyncSchemaConnection;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = diesel_async::AsyncMysqlConnection::establish(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations via AsyncConnectionWrapper in a blocking task
    let url = database_url.clone();
    tokio::task::spawn_blocking(move || {
        let mut sync_conn =
            <AsyncConnectionWrapper<diesel_async::AsyncMysqlConnection> as diesel::Connection>::establish(&url)
                .expect("Failed to establish sync wrapper connection");
        sync_conn
            .run_pending_migrations(MYSQL_MIGRATIONS)
            .expect("Failed to run migrations");
    })
    .await
    .expect("Blocking task panicked");

    // Get all books and pages in the default database
    use data_mysql::schema::{books, pages};
    let default_books = books::table
        .load::<(i32, String)>(&mut connection)
        .await
        .expect("Failed to load books from default schema");
    assert_eq!(
        default_books.len(),
        0,
        "Expected no books in default schema"
    );
    let default_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .await
        .expect("Failed to load pages from default schema");
    assert_eq!(
        default_pages.len(),
        0,
        "Expected no pages in default schema"
    );

    // Switch to a new database
    connection
        .set_schema("async_new_mysql_schema")
        .await
        .expect("Failed to switch schema");

    // Run migrations in the new database via AsyncConnectionWrapper
    let url = database_url.clone();
    tokio::task::spawn_blocking(move || {
        use diesel::connection::SimpleConnection;
        let mut sync_conn =
            <AsyncConnectionWrapper<diesel_async::AsyncMysqlConnection> as diesel::Connection>::establish(&url)
                .expect("Failed to establish sync wrapper connection");
        sync_conn
            .batch_execute("CREATE DATABASE IF NOT EXISTS `async_new_mysql_schema`")
            .expect("Failed to create database");
        sync_conn
            .batch_execute("USE `async_new_mysql_schema`")
            .expect("Failed to switch database");
        sync_conn
            .run_pending_migrations(MYSQL_MIGRATIONS)
            .expect("Failed to run migrations in new schema");
    })
    .await
    .expect("Blocking task panicked");

    // Get all books and pages in the new database (should be empty)
    let new_books = books::table
        .load::<(i32, String)>(&mut connection)
        .await
        .expect("Failed to load books from new schema");
    assert_eq!(new_books.len(), 0, "Expected no books in new schema");
    let new_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .await
        .expect("Failed to load pages from new schema");
    assert_eq!(new_pages.len(), 0, "Expected no pages in new schema");
}
