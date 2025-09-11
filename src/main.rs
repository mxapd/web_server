mod html;
mod http;

use http::handle_client;
use std::io::Result;
use std::net::TcpListener;

// TODO: Add logging

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2323")?;

    for stream in listener.incoming() {
        let _ = handle_client(stream?);
    }

    Ok(())
}
