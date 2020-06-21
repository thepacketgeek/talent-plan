//# anyhow = ""
//# serde = { version = "", features = ["derive"] }
//# serde_json = ""

use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Move {
    x: i32,
    y: i32,
}

impl Move {
    pub fn new(x: i32, y: i32) -> Self {
        Move { x, y }
    }

    pub fn to_bytes_json(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        let writer = BufWriter::new(&mut bytes);
        serde_json::to_writer(writer, &self).with_context(|| format!("Writing {:?}", &self))?;
        Ok(bytes)
    }

    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = OpenOptions::new().read(true).open(path)?;
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).with_context(|| format!("Reading"))
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = OpenOptions::new().write(true).create(true).open(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self).with_context(|| format!("Writing {:?}", &self))
    }
}

fn main() -> Result<()> {
    let a = Move::new(1, 2);
    println!("Original:     {:?}", a);

    // JSON
    a.write_to_file("tmp.data")?;
    let b = Move::read_from_file("tmp.data")?;
    println!("From JSON:    {:?}", b);

    // Bytes
    let bytes = a.to_bytes_json();
    println!("JSON Bytes:   {:?}", bytes);

    Ok(())
}
