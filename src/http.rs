use std::collections::HashMap;

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub struct HttpResponse {
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    status_code: HttpStatus,
    // maybe no need since we can derive from status reason_phrase: String,
}

pub enum HttpMethod {
    GET,
    POST,
}

pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}
