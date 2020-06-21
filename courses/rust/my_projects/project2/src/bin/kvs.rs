use std::env::current_dir;

use anyhow::Result;
use structopt::StructOpt;

use kvs::{Command, KvStore};

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

    let mut store = KvStore::open(&current_dir()?)?;

    let res = match args.subcmd {
        Command::Get { key } => store
            .get(key)
            .map(|res| println!("{}", res.unwrap_or_else(|| "Key not found".to_owned()))),

        Command::Set { key, value } => store.set(key, value).map(|_| ()),
        Command::Remove { key } => store.remove(key).map(|_| ()),
    };
    if let Err(err) = res {
        println!("{}", err);
        std::process::exit(1);
    }
    Ok(())
}
