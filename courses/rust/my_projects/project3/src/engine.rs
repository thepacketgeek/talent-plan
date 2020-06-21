use crate::Result;

/// Trait for KVStore storage engines
pub trait KvsEngine {
    /// Set a key/value (insert if new, update if existing)
    fn set(&mut self, key: String, value: String) -> Result<()>;

    /// Get a key/value (if it exists)
    fn get(&self, key: String) -> Result<Option<String>>;

    /// Remove a key/value (if it exists)
    fn remove(&self, key: String) -> Result<()>;
}
