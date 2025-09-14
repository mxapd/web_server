use crate::http::http_request::HttpRequest;
use std::result::Result;
use std::sync::mpsc::Sender;

pub enum DatabaseMessage {
    Query {
        sql: String,
        response: Sender<Result<Vec<postgres::Row>, Box<dyn std::error::Error + Send + Sync>>>,
    },
    Shutdown,
}

pub enum RouterMessage {
    Route { http_request: HttpRequest },
}
