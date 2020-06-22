use std::io;
use std::path::PathBuf;

use crate::{KvsEngine, KvsError, Result};

/// The `SledKvsEngine` stores string key/value pairs
///
/// Key/value pairs are persisted to disk usign `sled` embedded db.
///
/// ```rust
/// # use kvs::{KvsEngine, Result, SledKvsEngine};
/// # fn try_main() -> Result<()> {
/// use std::env::current_dir;
/// let mut store = SledKvsEngine::open(current_dir()?)?;
/// store.set("key".to_owned(), "value".to_owned())?;
/// let val = store.get("key".to_owned())?;
/// assert_eq!(val, Some("value".to_owned()));
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct SledKvsEngine {
    path: PathBuf,
}

impl SledKvsEngine {
    /// Opens a `SledKvsEngine` with the given path.
    ///
    /// This will create a new Sled db if the given one does not exist.
    ///
    /// # Errors
    ///
    /// It propagates I/O or deserialization errors during the sled read.
    pub fn open(path: impl Into<PathBuf>) -> io::Result<Self> {
        Ok(Self { path: path.into() })
    }
}

impl KvsEngine for SledKvsEngine {
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    ///
    /// # Errors
    ///
    /// It propagates I/O or serialization errors during writing the sled-db.
    fn get(&mut self, key: String) -> Result<Option<String>> {
        todo!();
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    ///
    /// # Errors
    ///
    /// It propagates I/O or serialization errors during writing the sled-db.
    fn set(&mut self, key: String, value: String) -> Result<()> {
        todo!();
    }

    /// Removes a given key.
    ///
    /// # Errors
    ///
    /// It returns `KvsError::KeyNotFound` if the given key is not found.
    ///
    /// It propagates I/O or serialization errors during writing the sled-db.
    fn remove(&mut self, key: String) -> Result<()> {
        todo!();
    }
}
