use crate::handlers;
use crate::{actors::messages::RouterMessage, http::http_method::HttpMethod};

use std::sync::mpsc::Receiver;

pub struct RouterActor {
    mailbox: Receiver<RouterMessage>,
}

impl RouterActor {
    pub fn new(mailbox: Receiver<RouterMessage>) -> Self {
        Self { mailbox }
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
                    match (http_request.method, http_request.path.as_str()) {
                        (HttpMethod::GET, "/") => {
                            let response = handlers::home::home_page().unwrap();

                            let _ = response_tx.send(response);
                        }
                        _ => println!("Route not found!"),
                    };
                }
            }
        }
        println!("Router actor stopped!");
    }
}
