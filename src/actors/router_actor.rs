use crate::http::http_response::HttpResponse;
use crate::http::http_status::HttpStatus;
use std::sync::mpsc::Receiver;

use crate::html::Html;
use crate::{actors::messages::RouterMessage, http::http_method::HttpMethod};

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
                    let response = match (http_request.method, http_request.path.as_str()) {
                        (HttpMethod::GET, "/") => {
                            let html = Html::from_file("index.html".to_string());
                            println!("loaded file result: {:?}", html);

                            let html = html.unwrap();

                            let response = HttpResponse::from_html(html, HttpStatus::Ok);
                            println!("built response");

                            response_tx.send(response);
                        }
                        _ => println!("Route not found!"),
                    };
                }
            }
        }
        println!("Router actor stopped!");
    }
}
