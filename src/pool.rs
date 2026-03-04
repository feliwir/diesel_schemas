use std::collections::HashSet;
use std::ops::Deref;
use std::sync::RwLock;

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

/// A pool wrapper that caches which schemas have already been created,
/// avoiding the `CREATE SCHEMA IF NOT EXISTS` overhead on repeated calls.
pub struct CachedSchemaPool<M: ManageConnection> {
    pool: diesel::r2d2::Pool<M>,
    known_schemas: RwLock<HashSet<String>>,
}

impl<M: ManageConnection> CachedSchemaPool<M>
where
    M::Connection: SchemaConnection,
{
    pub fn new(pool: diesel::r2d2::Pool<M>) -> Self {
        Self {
            pool,
            known_schemas: RwLock::new(HashSet::new()),
        }
    }

    /// Returns the underlying pool.
    pub fn inner(&self) -> &diesel::r2d2::Pool<M> {
        &self.pool
    }
}

impl<M: ManageConnection> Deref for CachedSchemaPool<M> {
    type Target = diesel::r2d2::Pool<M>;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl<M: ManageConnection> SchemaPool<M> for CachedSchemaPool<M>
where
    M::Connection: SchemaConnection,
{
    fn get_with_schema(&self, schema: &str) -> QueryResult<PooledConnection<M>> {
        let mut conn = self.pool.get().map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::Unknown,
                Box::new(e.to_string()),
            )
        })?;

        // Fast path: check if schema is already known (read lock, no contention)
        let already_known = self
            .known_schemas
            .read()
            .expect("known_schemas lock poisoned")
            .contains(schema);

        if already_known {
            // Schema already exists — just switch the search path
            conn.set_search_path(schema)?;
        } else {
            // First time seeing this schema — create it and remember
            conn.set_schema(schema)?;
            self.known_schemas
                .write()
                .expect("known_schemas lock poisoned")
                .insert(schema.to_owned());
        }

        Ok(conn)
    }
}
