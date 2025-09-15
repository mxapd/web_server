use crate::html::html::Html;
use crate::http::http_status::HttpStatus;
use std::collections::HashMap;

#[derive(Clone)]
pub struct HttpResponse {
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub status_code: HttpStatus,
}

impl HttpResponse {
    pub fn from_html(html: Html, status: HttpStatus) -> Self {
        let html_bytes = html.into_bytes();

        let mut headers = HashMap::new();
        headers.insert("Content-Length".to_string(), html_bytes.len().to_string());
        headers.insert(String::from("Content-Type"), String::from("text/html"));

        HttpResponse {
            version: String::from("HTTP/1.1"),
            headers: headers,
            body: html_bytes,
            status_code: status,
        }
    }

    pub fn reason_phrase(&self) -> &str {
        match self.status_code {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::InternalServerError => "Internal Server Error",
        }
    }
}
