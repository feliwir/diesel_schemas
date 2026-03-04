use diesel::QueryResult;

use crate::conn::SchemaConnection;

impl SchemaConnection for diesel::MysqlConnection {
    fn set_schema(&mut self, schema: &str) -> QueryResult<usize> {
        self.ensure_schema(schema)?;
        self.set_search_path(schema)
    }

    fn set_search_path(&mut self, schema: &str) -> QueryResult<usize> {
        use diesel::connection::SimpleConnection;
        // USE is not supported in MySQL's prepared statement protocol,
        // so we use batch_execute to send raw SQL.
        self.batch_execute(&format!("USE `{schema}`"))?;
        Ok(0)
    }

    fn ensure_schema(&mut self, schema: &str) -> QueryResult<usize> {
        use diesel::connection::SimpleConnection;
        // CREATE DATABASE is not supported in MySQL's prepared statement protocol,
        // so we use batch_execute to send raw SQL.
        self.batch_execute(&format!("CREATE DATABASE IF NOT EXISTS `{schema}`"))?;
        Ok(0)
    }
}
