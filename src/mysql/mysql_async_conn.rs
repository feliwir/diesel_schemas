use diesel::QueryResult;

use crate::async_conn::AsyncSchemaConnection;

impl AsyncSchemaConnection for diesel_async::AsyncMysqlConnection {
    async fn set_schema(&mut self, schema: &str) -> QueryResult<()> {
        self.ensure_schema(schema).await?;
        self.set_search_path(schema).await
    }

    async fn set_search_path(&mut self, schema: &str) -> QueryResult<()> {
        use diesel_async::SimpleAsyncConnection;
        self.batch_execute(&format!("USE `{schema}`")).await
    }

    async fn ensure_schema(&mut self, schema: &str) -> QueryResult<()> {
        use diesel_async::SimpleAsyncConnection;
        self.batch_execute(&format!("CREATE DATABASE IF NOT EXISTS `{schema}`"))
            .await
    }
}
