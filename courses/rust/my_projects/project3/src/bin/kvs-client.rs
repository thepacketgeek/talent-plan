use std::env::current_dir;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::process::exit;

use log::trace;
use structopt::StructOpt;

use kvs::{KvStore, KvsError, Request};

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
    /// Server listening address
    #[structopt(long, default_value = "127.0.0.1:4000")]
    addr: SocketAddr,
}

fn send_request(addr: SocketAddr, req: &Request) -> io::Result<()> {
    dbg!(&req);
    let mut stream = TcpStream::connect(addr)?;
    let bytes_sent = stream.write(&req.serialize()?)?;
    stream.flush()?;
    eprintln!("Sent {} bytes", bytes_sent);

    let mut received = vec![];
    stream.read(&mut received)?;
    eprintln!("Received {} bytes", received.len());
    Ok(())
}

fn main() -> Result<(), String> {
    let args = Args::from_args();

    match args.subcmd {
        Command::Get { key } => {
            let req = Request::Get { key };
            send_request(args.addr, &req).map_err(|e| format!("Error fetching key: {}", e))
        }
        Command::Set { key, value } => {
            let req = Request::Set { key, value };
            send_request(args.addr, &req).map_err(|e| format!("Error setting key: {}", e))
        }
        Command::Remove { key } => {
            let req = Request::Remove { key };
            send_request(args.addr, &req).map_err(|e| format!("Error removing key: {}", e))
        }
    }
}
