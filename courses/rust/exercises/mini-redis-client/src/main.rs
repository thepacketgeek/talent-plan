use anyhow::Result;
use mini_redis_proto::{Request, Response};
use tokio::net::TcpStream;
use tokio::prelude::*;
use tokio::time::{delay_for, Duration};

async fn ping(addr: &str) -> Result<()> {
    let mut stream = TcpStream::connect(&addr).await?;
    let req: String = serde_json::to_string(&Request::Ping)?;
    stream.write(&req.as_bytes()).await?;
    let mut buf = [0; 64];
    let bytes_read = stream.read(&mut buf).await?;
    let resp: Response = serde_json::from_str(std::str::from_utf8(&buf[..bytes_read])?)?;
    println!("wrote to stream; num bytes={:?}", resp);
    Ok(())
}

#[tokio::main]
async fn main() {
    loop {
        ping("127.0.0.1:6379")
            .await
            .map_err(|err| eprintln!("ERROR: {}", err))
            .and_then(|_| {
                println!("PING Success!");
                Ok(())
            });
        delay_for(Duration::from_secs(3)).await;
    }
}
