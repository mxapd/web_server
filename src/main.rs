mod html;
mod http;
mod request_actor;

use request_actor::RequestActor;
use std::io::Result;
use std::net::TcpListener;
use std::thread;

// TODO: Add logging

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2323")?;

    for stream in listener.incoming() {
        thread::spawn(move || {
            let actor = RequestActor::new(stream.unwrap());
            actor.run().unwrap();
        });
    }

    Ok(())
}
