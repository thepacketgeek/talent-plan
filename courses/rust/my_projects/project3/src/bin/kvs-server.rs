use std::env::current_dir;
use std::net::SocketAddr;
use std::process::exit;

use structopt::StructOpt;

use kvs::{KvStore, KvsError, Result};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kvs-server",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
)]
struct Args {
    /// Service listening address
    #[structopt(long, default_value = "127.0.0.1:4000")]
    addr: SocketAddr,
    /// Storage engine to use (["kvs", "sled"])
    #[structopt(long, default_value = "kvs")]
    engine: Engine,
}

#[derive(Debug)]
enum Engine {
    Kvs,
    Sled,
}

impl std::str::FromStr for Engine {
    type Err = std::io::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "kvs" => Ok(Engine::Kvs),
            "sled" => Ok(Engine::Sled),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Engine must be one of ('kvs', 'sled')",
            )),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();
    dbg!(&args);

    // match args.subcmd {
    //     Command::Get { key } => {
    //         let mut store = KvStore::open(current_dir()?)?;
    //         if let Some(value) = store.get(key.to_string())? {
    //             println!("{}", value);
    //         } else {
    //             println!("Key not found");
    //         }
    //     }
    //     Command::Set { key, value } => {
    //         let mut store = KvStore::open(current_dir()?)?;
    //         store.set(key.to_string(), value.to_string())?;
    //     }
    //     Command::Remove { key } => {
    //         let mut store = KvStore::open(current_dir()?)?;
    //         match store.remove(key.to_string()) {
    //             Ok(()) => {}
    //             Err(KvsError::KeyNotFound) => {
    //                 println!("Key not found");
    //                 exit(1);
    //             }
    //             Err(e) => return Err(e),
    //         }
    //     }
    // }
    Ok(())
}
