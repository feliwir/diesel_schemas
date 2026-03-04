use diesel_migrations::{EmbeddedMigrations, embed_migrations};

pub const PG_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./tests/data_pg/migrations");
mod data_pg;

#[test]
#[cfg(feature = "postgres")]
fn switch_schema() {
    use diesel::prelude::*;
    use diesel_migrations::MigrationHarness;
    use diesel_schemas::conn::SchemaConnection;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection =
        diesel::PgConnection::establish(&database_url).expect("Failed to connect to database");

    // Create the "books" and "pages" tables in the default schema
    connection
        .run_pending_migrations(PG_MIGRATIONS)
        .expect("Failed to run migrations");

    // Get all books and pages in the default schema
    use data_pg::schema::{books, pages};
    let default_books = books::table
        .load::<(i32, String)>(&mut connection)
        .expect("Failed to load books from default schema");
    assert_eq!(
        default_books.len(),
        0,
        "Expected no books in default schema"
    );
    let default_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .expect("Failed to load pages from default schema");
    assert_eq!(
        default_pages.len(),
        0,
        "Expected no pages in default schema"
    );

    // Create a new schema and switch to it
    connection
        .set_schema("new_schema")
        .expect("Failed to switch schema");

    // Rerun all pending migrations in the new schema (should create the tables there)
    connection
        .run_pending_migrations(PG_MIGRATIONS)
        .expect("Failed to run migrations in new schema");

    // Get all books and pages in the new schema (should be empty)
    let new_books = books::table
        .load::<(i32, String)>(&mut connection)
        .expect("Failed to load books from new schema");
    assert_eq!(new_books.len(), 0, "Expected no books in new schema");
    let new_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .expect("Failed to load pages from new schema");
    assert_eq!(new_pages.len(), 0, "Expected no pages in new schema");
}

pub const MYSQL_MIGRATIONS: EmbeddedMigrations = embed_migrations!("./tests/data_mysql/migrations");
mod data_mysql;

#[test]
#[cfg(feature = "mysql")]
fn switch_schema_mysql() {
    use diesel::prelude::*;
    use diesel_migrations::MigrationHarness;
    use diesel_schemas::conn::SchemaConnection;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection =
        diesel::MysqlConnection::establish(&database_url).expect("Failed to connect to database");

    // Create the "books" and "pages" tables in the default database
    connection
        .run_pending_migrations(MYSQL_MIGRATIONS)
        .expect("Failed to run migrations");

    // Get all books and pages in the default database
    use data_mysql::schema::{books, pages};
    let default_books = books::table
        .load::<(i32, String)>(&mut connection)
        .expect("Failed to load books from default schema");
    assert_eq!(
        default_books.len(),
        0,
        "Expected no books in default schema"
    );
    let default_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .expect("Failed to load pages from default schema");
    assert_eq!(
        default_pages.len(),
        0,
        "Expected no pages in default schema"
    );

    // Create a new database and switch to it
    connection
        .set_schema("new_mysql_schema")
        .expect("Failed to switch schema");

    // Rerun all pending migrations in the new database (should create the tables there)
    connection
        .run_pending_migrations(MYSQL_MIGRATIONS)
        .expect("Failed to run migrations in new schema");

    // Get all books and pages in the new database (should be empty)
    let new_books = books::table
        .load::<(i32, String)>(&mut connection)
        .expect("Failed to load books from new schema");
    assert_eq!(new_books.len(), 0, "Expected no books in new schema");
    let new_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .expect("Failed to load pages from new schema");
    assert_eq!(new_pages.len(), 0, "Expected no pages in new schema");
}
