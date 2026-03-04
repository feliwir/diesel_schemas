use diesel::QueryResult;

pub trait AsyncSchemaConnection {
    /// Creates the schema if it doesn't exist and sets the search path.
    fn set_schema(&mut self, schema: &str) -> impl Future<Output = QueryResult<()>> + Send;

    /// Only sets the search path without creating the schema.
    /// Use this when you know the schema already exists.
    fn set_search_path(&mut self, schema: &str) -> impl Future<Output = QueryResult<()>> + Send;

    /// Creates the schema if it doesn't exist (without switching to it).
    fn ensure_schema(&mut self, schema: &str) -> impl Future<Output = QueryResult<()>> + Send;
}
