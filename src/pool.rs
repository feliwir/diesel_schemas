use diesel::{
    QueryResult,
    r2d2::{ManageConnection, PooledConnection},
};

use crate::conn::SchemaConnection;

pub trait SchemaPool<M: ManageConnection>
where
    M::Connection: SchemaConnection,
{
    fn get_with_schema(&self, schema: &str) -> QueryResult<PooledConnection<M>>;
}

impl<M: ManageConnection> SchemaPool<M> for diesel::r2d2::Pool<M>
where
    M::Connection: SchemaConnection,
{
    fn get_with_schema(&self, schema: &str) -> QueryResult<PooledConnection<M>> {
        let mut conn = self.get().map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            )
        })?;
        conn.set_schema(schema)?;
        Ok(conn)
    }
}
