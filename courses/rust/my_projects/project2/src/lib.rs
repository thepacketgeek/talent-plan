// #![deny(missing_docs)]

//! A String-focused implementation of a Key/Value store
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

pub use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use structopt::StructOpt;

/// Options for managing KVStore values
#[derive(Clone, Debug, Deserialize, Serialize, StructOpt)]
pub enum Command {
    /// Get a stored value by key
    Get { key: String },
    /// Set a stored value by key
    Set { key: String, value: String },
    /// Remove a stored value by key
    #[structopt(name = "rm")]
    Remove { key: String },
}

/// A Key-Value store for Strings
pub struct KvStore {
    inner: HashMap<String, String>,
    writer: BufWriter<File>,
}

/// Methods for managing stored values in KvStore
impl KvStore {
    /// Create a KvStore, populated with events from the given file
    /// ```rust
    /// use kvs::KvStore;
    /// let store = KvStore::open("tmp.log")?;
    /// ```
    pub fn open(path: &Path) -> Result<Self> {
        let log_path = path.join("kvs.log");
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)?;

        let logs = fs::read_to_string(&log_path)?;
        let mut store = KvStore {
            inner: HashMap::new(),
            writer: BufWriter::new(file),
        };
        for line in logs.lines() {
            let command: Command = serde_json::from_str(&line)?;
            store.execute(command)?;
        }
        Ok(store)
    }

    /// Store a value, to be retrieved by a given key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("MyKey".to_owned(), "MyValue".to_owned())?;
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::Set { key, value };
        // TODO: Cleanup old "Set"/Remove for this key?
        // Store the command in our log file
        write_command_to_file(&command, &mut self.writer)?;
        // Update in-memory store
        self.execute(command)
    }

    /// Retrieve a stored value (if exists) for a given key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("MyKey".to_owned(), "MyValue".to_owned());
    ///
    /// assert_eq!(store.get("MyKey".to_owned()), Some("MyValue".to_owned()));
    /// ```
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.inner.get(&key).map(|v| v.clone()))
    }

    /// Remove a stored value for a given key
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("MyKey".to_owned(), "MyValue".to_owned());
    ///
    /// store.remove("MyKey".to_owned())?;
    /// assert_eq!(store.get("MyKey".to_owned()), None);
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.inner.contains_key(&key) {
            return Err(anyhow!("Key not found"));
        }
        let command = Command::Remove { key };
        // TODO: Cleanup old "Set"/Remove for this key?
        // Store the command in our log file
        write_command_to_file(&command, &mut self.writer)?;
        // Update in-memory store
        self.execute(command)
    }

    /// Store the log to file, populating the internal K/V store
    fn execute(&mut self, command: Command) -> Result<()> {
        // Update in-memory KVStore
        match command {
            Command::Set { key, value } => {
                self.inner.insert(key, value);
            }
            Command::Remove { key } => {
                self.inner.remove(&key);
            }
            _ => (),
        }
        Ok(())
    }
}

/// Append a Command log to the end of the given filepath
fn write_command_to_file(command: &Command, writer: &mut BufWriter<File>) -> Result<()> {
    let log = serde_json::to_string(&command)?;
    writer.write(format!("{}\n", log).as_bytes())?;
    writer.flush()?;
    Ok(())
}
