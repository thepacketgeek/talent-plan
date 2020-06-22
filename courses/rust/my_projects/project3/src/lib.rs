#![deny(missing_docs)]
//! A simple key/value store.

pub use crate::sled::SledKvsEngine;
pub use error::{KvsError, Result};
pub use kv::KvStore;
pub use proto::{Request, Response};

mod error;
mod kv;
mod proto;
mod sled;

/// Trait for KVStore storage engines
pub trait KvsEngine {
    /// Set a key/value (insert if new, update if existing)
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Get a key/value (if it exists)
    fn get(&mut self, key: String) -> Result<Option<String>>;

    /// Remove a key/value (if it exists)
    fn remove(&mut self, key: String) -> Result<()>;
}
