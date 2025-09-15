use crate::actors::actor_directory::ActorDirectory;
use crate::actors::messages::HandlerMessage;
use crate::handlers;
use crate::html::html_builder::HtmlBuilder;

use std::sync::mpsc::Receiver;
use std::sync::{Arc, mpsc};

pub struct HandlerActor {
    mailbox: Receiver<HandlerMessage>,
    actor_directory: Arc<ActorDirectory>,
}

impl HandlerActor {
    pub fn new(mailbox: Receiver<HandlerMessage>, actor_directory: Arc<ActorDirectory>) -> Self {
        Self {
            mailbox,
            actor_directory,
        }
    }

    pub fn run(self) {
        println!("Handler actor started");

        for message in self.mailbox {
            match message {
                HandlerMessage::Home {
                    request,
                    response_tx,
                } => {
                    let response = handlers::home::handle_request();
                    let _ = response_tx.send(response.unwrap());
                }

                HandlerMessage::Articles {
                    request,
                    response_tx,
                } => {
                    let response = handlers::articles::handle_list_articles_request();
                }

                _ => println!("Handler Actor: unknown message"),
            }
        }
    }
}
