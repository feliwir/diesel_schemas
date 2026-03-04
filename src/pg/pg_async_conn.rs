use diesel::QueryResult;

use crate::async_conn::AsyncSchemaConnection;

impl AsyncSchemaConnection for diesel_async::AsyncPgConnection {
    async fn set_schema(&mut self, schema: &str) -> QueryResult<()> {
        self.ensure_schema(schema).await?;
        self.set_search_path(schema).await
    }

    async fn set_search_path(&mut self, schema: &str) -> QueryResult<()> {
        use diesel_async::SimpleAsyncConnection;
        self.batch_execute(&format!("SET SEARCH_PATH TO {schema}"))
            .await
    }

    async fn ensure_schema(&mut self, schema: &str) -> QueryResult<()> {
        use diesel_async::SimpleAsyncConnection;
        self.batch_execute(&format!("CREATE SCHEMA IF NOT EXISTS {schema}"))
            .await
    }
}
