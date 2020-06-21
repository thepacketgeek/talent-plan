use std::convert::From;
use std::io::{self, Cursor, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

/// Request object (client -> server)
#[derive(Debug)]
pub enum Request {
    /// Fetch a value for a given key
    Get {
        /// Requested key
        key: String,
    },
    /// Set (or update) a key/value pair
    Set {
        /// key to set
        key: String,
        /// value to set
        value: String,
    },
    /// Remove a key/value pair (if it exists)
    Remove {
        /// key to remove
        key: String,
    },
}

/// Request type byte
impl From<&Request> for u8 {
    fn from(req: &Request) -> Self {
        match req {
            Request::Get { .. } => 1,
            Request::Set { .. } => 2,
            Request::Remove { .. } => 3,
        }
    }
}

impl Request {
    /// Serialize Request to bytes (to send to server)
    pub fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut data: Vec<u8> = vec![];
        data.write_u8(self.into())?;
        match self {
            Request::Get { key } | Request::Remove { key } => {
                let key_bytes = key.as_bytes();
                data.write_u16::<BigEndian>(key_bytes.len() as u16)?;
                data.write_all(&key_bytes)?;
            }
            Request::Set { key, value } => {
                let key_bytes = key.as_bytes();
                data.write_u16::<BigEndian>(key_bytes.len() as u16)?;
                data.write_all(&key_bytes)?;
                let value_bytes = value.as_bytes();
                data.write_u16::<BigEndian>(value_bytes.len() as u16)?;
                data.write_all(&value_bytes)?;
            }
        }
        Ok(data)
    }

    /// Deerialize Request to bytes (to receive from client)
    pub fn deserialize(data: &[u8]) -> io::Result<Request> {
        let mut cursor = Cursor::new(data);
        match cursor.read_u8()? {
            // Get
            1 => {
                let key = extract_string(&mut cursor)?;
                Ok(Request::Get { key })
            }
            // Set
            2 => {
                let key = extract_string(&mut cursor)?;
                let value = extract_string(&mut cursor)?;
                Ok(Request::Set { key, value })
            }
            // Remove
            3 => {
                let key = extract_string(&mut cursor)?;
                Ok(Request::Remove { key })
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Request Type",
            )),
        }
    }
}

/// Response object (server)
#[derive(Debug)]
pub enum Response {
    /// A fetched Value
    /// Response for Get (if key exists)
    Value(String),
    /// Request was processed, nothing to return
    /// Response for Remove/Set
    Ok,
    /// Key does not exist (for Get/Remove), or other error
    Error(String),
}

/// Request type byte
impl From<&Response> for u8 {
    fn from(req: &Response) -> Self {
        match req {
            Response::Value(_) => 1,
            Response::Ok => 2,
            Response::Error(_) => 3,
        }
    }
}

impl Response {
    /// Serialize Response to bytes (to send to client)
    pub fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut data: Vec<u8> = vec![];
        data.write_u8(self.into())?;
        match self {
            Response::Value(resp) | Response::Error(resp) => {
                let resp_bytes = resp.as_bytes();
                data.write_u16::<BigEndian>(resp_bytes.len() as u16)?;
                data.write_all(&resp_bytes)?;
            }
            Response::Ok => (),
        }
        Ok(data)
    }

    /// Deerialize Response to bytes (to receive from server)
    pub fn deserialize(data: &[u8]) -> io::Result<Response> {
        let mut cursor = Cursor::new(data);
        match cursor.read_u8()? {
            // Value
            1 => {
                let value = extract_string(&mut cursor)?;
                Ok(Response::Value(value))
            }
            // Ok
            2 => Ok(Response::Ok),
            // Error
            3 => {
                let error = extract_string(&mut cursor)?;
                Ok(Response::Error(error))
            }
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid Response Type",
            )),
        }
    }
}

/// From a given Cursor, read the next length (u16) and extract the string bytes
fn extract_string(cursor: &mut Cursor<&[u8]>) -> io::Result<String> {
    let length = cursor.read_u16::<BigEndian>()?;
    let mut bytes = vec![0u8; length as usize];
    cursor.read_exact(&mut bytes)?;
    String::from_utf8(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8"))
}
