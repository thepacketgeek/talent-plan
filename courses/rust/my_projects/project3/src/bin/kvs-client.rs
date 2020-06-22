use std::io::{self, BufReader, BufWriter, Write};
use std::net::{SocketAddr, TcpStream};

use structopt::StructOpt;

use kvs::{Request, Response};

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Get a stored value by key
    Get { key: String },
    /// Set a stored value by key
    Set { key: String, value: String },
    /// Remove a stored value by key
    #[structopt(name = "rm", alias = "remove")]
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
    #[structopt(long, default_value = "127.0.0.1:4000", global = true)]
    addr: SocketAddr,
}

struct KvsClient {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    fn connect(dest: SocketAddr) -> io::Result<Self> {
        let stream = TcpStream::connect(dest)?;
        eprintln!("Connecting to {}", dest);
        Ok(Self {
            reader: BufReader::new(stream.try_clone()?),
            writer: BufWriter::new(stream),
        })
    }

    fn send_request(&mut self, req: &Request) -> io::Result<Response> {
        self.writer.write_all(&req.serialize()?)?;
        self.writer.flush()?;
        Response::deserialize(&mut self.reader)
    }
}

fn main() -> Result<(), String> {
    let args = Args::from_args();

    let resp = match args.subcmd {
        Command::Get { key } => {
            let req = Request::Get { key };
            KvsClient::connect(args.addr)
                .and_then(|mut client| client.send_request(&req))
                .map_err(|e| format!("Error fetching key: {}", e))?
        }
        Command::Set { key, value } => {
            let req = Request::Set { key, value };
            KvsClient::connect(args.addr)
                .and_then(|mut client| client.send_request(&req))
                .map_err(|e| format!("Error setting key: {}", e))?
        }
        Command::Remove { key } => {
            let req = Request::Remove { key };
            KvsClient::connect(args.addr)
                .and_then(|mut client| client.send_request(&req))
                .map_err(|e| format!("Error removing key: {}", e))?
        }
    };
    match resp {
        Response::Value(v) => println!("{}", v),
        Response::Ok => (),
        Response::Error(err) => println!("{}", err),
    };
    Ok(())
}
