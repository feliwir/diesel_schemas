use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use diesel_schemas::SchemaConnection;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./tests/data/migrations");

mod data;

#[test]
fn switch_schema() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection =
        diesel::PgConnection::establish(&database_url).expect("Failed to connect to database");

    // Create the "books" and "pages" tables in the default schema
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    // Get all books and pages in the default schema
    use data::schema::{books, pages};
    let default_books = books::table
        .load::<(i32, String)>(&mut connection)
        .expect("Failed to load books from default schema");
    let default_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .expect("Failed to load pages from default schema");

    // Create a new schema and switch to it
    connection
        .set_schema("new_schema")
        .expect("Failed to switch schema");

    // Rerun all pending migrations in the new schema (should create the tables there)
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations in new schema");

    // Get all books and pages in the new schema (should be empty)
    let new_books = books::table
        .load::<(i32, String)>(&mut connection)
        .expect("Failed to load books from new schema");
    let new_pages = pages::table
        .load::<(i32, i32, Option<String>)>(&mut connection)
        .expect("Failed to load pages from new schema");
}
