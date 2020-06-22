use std::env::current_dir;
use std::fmt;
use std::io::{self, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpListener};

use log::{debug, error, info, trace};
use structopt::StructOpt;

use kvs::{KvStore, KvsEngine, KvsError, Request, Response, Result, SledKvsEngine};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "kvs-server",
    about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
)]
struct Args {
    /// Storage engine to use (["kvs", "sled"])
    #[structopt(long, default_value = "kvs")]
    engine: Engine,
    /// Service listening address
    #[structopt(long, default_value = "127.0.0.1:4000", global = true)]
    addr: SocketAddr,
}

#[derive(Debug)]
enum Engine {
    Kvs,
    Sled,
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Self::Kvs => "kvs",
            Self::Sled => "sled",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for Engine {
    type Err = io::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "kvs" => Ok(Engine::Kvs),
            "sled" => Ok(Engine::Sled),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Engine must be one of ('kvs', 'sled')",
            )),
        }
    }
}

fn handle_request(req: Request, store: &mut Box<dyn KvsEngine>) -> Result<Response> {
    match req {
        Request::Get { key } => {
            info!("[GET] {}", key);
            if let Some(value) = store.get(key.to_string())? {
                Ok(Response::Value(value))
            } else {
                error!("Key '{}' not found", key);
                Ok(Response::Error(format!("Key '{}' not found", key)))
            }
        }
        Request::Set { key, value } => {
            info!("[SET] {} '{}'", key, value);
            store.set(key.to_string(), value.to_string())?;
            Ok(Response::Ok)
        }
        Request::Remove { key } => {
            info!("[RM] {}", key);
            match store.remove(key.to_string()) {
                Ok(()) => Ok(Response::Ok),
                Err(KvsError::KeyNotFound) => {
                    error!("Key '{}' not found", key);
                    Ok(Response::Error(format!("Key '{}' not found", key)))
                }
                Err(e) => Ok(Response::Error(e.to_string())),
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();
    env_logger::init();

    debug!(
        "Starting server on '{}' using '{}' engine",
        args.addr, args.engine
    );

    let cur_dir = current_dir()?;
    let mut engine: Box<dyn KvsEngine> = match &args.engine {
        Engine::Kvs => {
            if SledKvsEngine::has_existing_files(&cur_dir)? {
                return Err(KvsError::Io(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "Server was previously running with SledKvsEngine",
                )));
            }
            let store = KvStore::open(cur_dir)?;
            Box::new(store)
        }
        Engine::Sled => {
            if KvStore::has_existing_files(&cur_dir)? {
                return Err(KvsError::Io(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "Server was previously running with KvStore",
                )));
            }
            let store = SledKvsEngine::open(current_dir()?)?;
            Box::new(store)
        }
    };

    let listener = TcpListener::bind(args.addr)?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            trace!("Incoming from {}", stream.peer_addr().unwrap());

            let mut reader = BufReader::new(stream.try_clone()?);
            let mut writer = BufWriter::new(stream);

            match Request::deserialize(&mut reader) {
                Ok(req) => match handle_request(req, &mut engine) {
                    Ok(resp) => {
                        writer.write_all(&resp.serialize()?)?;
                        writer.flush()?;
                    }
                    Err(e) => {
                        error!("{}", e);
                        let resp = Response::Error(e.to_string());
                        writer.write_all(&resp.serialize()?)?;
                        writer.flush()?;
                    }
                },
                Err(e) => error!("Invalid Request: {}", e),
            }
        }
    }
    Ok(())
}
