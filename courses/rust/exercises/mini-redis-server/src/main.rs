
use anyhow::{anyhow, Result};
use futures::stream::StreamExt;
use mini_redis_proto::{Request, Response};
use tokio::net::TcpListener;
use tokio::prelude::*;


async fn handle_request(buf: &[u8]) -> Result<Option<Response>> {
    let raw_data = std::str::from_utf8(&buf)?;
    match serde_json::from_str(raw_data)? {
        Request::Ping => {
            eprintln!("Received PING");
            Ok(Some(Response::Pong))
        }
        _ => {
            Err(anyhow!("Unsupported Request: {:?}", raw_data))
        }
    }
}


#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = "127.0.0.1:6379";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(conn) = incoming.next().await {
            match conn {
                Ok(mut socket) => {
                    eprintln!("Accepted connection from {:?}", socket.peer_addr());
                    tokio::spawn(async move {
                        let mut buf = [0; 64];
                        let bytes_read = socket.read(&mut buf).await.unwrap();
                        match handle_request(&buf[..bytes_read]).await {
                            Ok(result) => {
                                if let Some(response) = result {
                                    let resp: String = serde_json::to_string(&response).unwrap();
                                    socket.write(&resp.as_bytes()).await.unwrap();
                                }
                            }
                            Err(err) => {
                                println!("ERROR: {}", err);
                            }
                        }
                    });
                }
                Err(err) => {
                    // Handle error by printing to STDOUT.
                    println!("accept error = {:?}", err);
                }
            }
        }
    };

    println!("Server listening on {}...", addr);
    server.await;
}
