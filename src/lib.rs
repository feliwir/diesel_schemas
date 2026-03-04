//! A library for managing connections and communication between different components of a system.
pub mod conn;
#[cfg(feature = "r2d2")]
pub mod pool;
