use kvs::KvStore;
use structopt::StructOpt;

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
    // Show version info
    // #[structopt(short = "-V", long)]
    // version: bool,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Get a stored value by key
    Get { key: String },
    /// Set a stored value by key
    Set { key: String, value: String },
    /// Remove a stored value by key
    #[structopt(name = "rm")]
    Remove { key: String },
}


fn main() {
    let args = Args::from_args();

    let mut store = KvStore::new();

    match args.subcmd {
        Command::Get { key } => {
            // store.get(key);
            todo!("unimplemented");
        }
        Command::Set { key, value } => {
            // store.set(key, value);
            todo!("unimplemented");
        }
        Command::Remove { key } => {
            // store.remove(key);
            todo!("unimplemented");
        }
    }
}
