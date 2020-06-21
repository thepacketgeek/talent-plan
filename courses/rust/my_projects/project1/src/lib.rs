#![deny(missing_docs)]

//! A String-focused implementation of a Key/Value store

use std::collections::HashMap;

/// A Key-Value store for Strings
pub struct KvStore(HashMap<String, String>);

/// Methods for managing stored values in KvStore
impl KvStore {
    /// Create an empty KvStore
    /// ```rust
    /// use kvs::KvStore;
    /// let store = KvStore::new();
    /// ```
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    
    /// Store a value, to be retrieved by a given key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("MyKey".to_owned(), "MyValue".to_owned());
    /// ```
    pub fn set(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }
    
    /// Retrieve a stored value (if exists) for a given key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("MyKey".to_owned(), "MyValue".to_owned());
    ///
    /// assert_eq!(store.get("MyKey".to_owned()), Some("MyValue".to_owned()));
    /// ```
    pub fn get(&self, key: String) -> Option<String> {
        self.0.get(&key).map(|v| v.clone())
    }

    /// Remove a stored value for a given key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("MyKey".to_owned(), "MyValue".to_owned());
    ///
    /// store.remove("MyKey".to_owned());
    /// assert_eq!(store.get("MyKey".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) {
        self.0.remove(&key);
    }
}
