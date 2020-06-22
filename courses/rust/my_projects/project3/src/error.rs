use std::io;
use thiserror::Error;

/// Error type for kvs.
#[derive(Error, Debug)]
pub enum KvsError {
    /// IO error.
    #[error("{0}")]
    Io(io::Error),
    /// Serialization or deserialization error.
    #[error("{0}")]
    Serde(serde_json::Error),
    /// Removing non-existent key error.
    #[error("Key not found")]
    KeyNotFound,
    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    #[error("Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> KvsError {
        match err {
            sled::Error::Io(err) => KvsError::Io(err),
            sled::Error::Corruption { .. } => KvsError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "Sled Data Corruption",
            )),
            e => KvsError::Io(io::Error::new(
                io::ErrorKind::Other,
                format!("Sled Error: {}", e),
            )),
        }
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;
