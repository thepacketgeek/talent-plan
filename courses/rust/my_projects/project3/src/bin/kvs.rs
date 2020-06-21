use std::env::current_dir;
use std::process::exit;

use structopt::StructOpt;

use kvs::{KvStore, KvsError, Result};

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Get a stored value by key
    Get { key: String },
    /// Set a stored value by key
    Set { key: String, value: String },
    /// Remove a stored value by key
    #[structopt(name = "rm")]
    Remove { key: String },
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kvs",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
)]
struct Args {
    #[structopt(subcommand)]
    subcmd: Command,
}

fn main() -> Result<()> {
    let args = Args::from_args();

    match args.subcmd {
        Command::Get { key } => {
            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Command::Set { key, value } => {
            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())?;
        }
        Command::Remove { key } => {
            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
    }
    Ok(())
}
