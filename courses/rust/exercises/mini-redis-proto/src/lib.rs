use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Request {
    Ping,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    Pong,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_request_ping() {
        let req = Request::Ping;
        let mut data = Vec::new();
        serde_json::to_writer(&mut data, &req).unwrap();
        eprintln!("{:?}", data);
    }

    #[test]
    fn test_request_pong() {
        let resp = Response::Pong;
        let mut data = Vec::new();
        serde_json::to_writer(&mut data, &resp).unwrap();
        eprintln!("{:?}", data);
    }
}
