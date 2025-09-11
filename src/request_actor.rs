use std::net::TcpStream;

use crate::http::handle_client;

pub struct RequestActor {
    stream: TcpStream,
}

impl RequestActor {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        handle_client(self.stream)
    }
}
