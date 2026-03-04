use diesel::QueryResult;

pub trait SchemaConnection {
    fn set_schema(&mut self, schema: &str) -> QueryResult<usize>;
}

#[cfg(feature = "postgres")]
impl SchemaConnection for diesel::PgConnection {
    fn set_schema(&mut self, schema: &str) -> QueryResult<usize> {
        use diesel::pg::{GetPgMetadataCache, PgMetadataCache};

        // Clear the metadata cache to ensure that the new schema is recognized
        *self.get_metadata_cache() = PgMetadataCache::new();

        // Create the schema if it doesn't exist
        use diesel::prelude::*;
        diesel::sql_query(format!("CREATE SCHEMA IF NOT EXISTS {schema}")).execute(self)?;

        // Set the search path to the specified schema for the current connection
        diesel::sql_query(format!("SET SEARCH_PATH TO {schema}")).execute(self)
    }
}
