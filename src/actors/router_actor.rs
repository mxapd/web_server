use crate::actors::actor_directory::ActorDirectory;
use crate::{
    actors::messages::{HandlerMessage, RouterMessage},
    http::http_method::HttpMethod,
};

use std::sync::Arc;

use std::sync::mpsc::Receiver;

pub struct RouterActor {
    mailbox: Receiver<RouterMessage>,
    actor_directory: Arc<ActorDirectory>,
}

impl RouterActor {
    pub fn new(mailbox: Receiver<RouterMessage>, actor_directory: Arc<ActorDirectory>) -> Self {
        Self {
            mailbox,
            actor_directory,
        }
    }

    pub fn run(self) {
        println!("Router actor started!");
        for message in self.mailbox {
            println!("Router received message!");
            match message {
                RouterMessage::Route {
                    http_request,
                    response_tx,
                } => {
                    match (&http_request.method, http_request.path.as_str()) {
                        (HttpMethod::GET, "/") => {
                            let _ = self.actor_directory.handler.send(HandlerMessage::Home {
                                request: http_request,
                                response_tx: response_tx,
                            });
                        }
                        _ => println!("Route not found!"),
                    };
                }
            }
        }
        println!("Router actor stopped!");
    }
}
