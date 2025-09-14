use crate::database_actor::PostgresActor;
use crate::http::handle_client;
use crate::messages::RequestMessage;
use std::io::Result;
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

pub struct RequestActor {
    sender: mpsc::Sender<RequestActor>,
    stream: TcpStream,
    database_actor: PostgresActor,
}

impl RequestActor {
    pub fn new(db_actor: PostgresActor) -> Self {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                match message {
                    RequestMessage::HandleHttp { stream, response } => {
                        let result = handle_client(stream, db_actor.clone());
                        let _ = response.send(result);
                    }
                    RequestMessage::Shutdown => break, // Clean exit
                }
            }
        });

        Self { sender }
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        handle_client(self.stream)
    }
}
