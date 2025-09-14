use crate::http::http_method::HttpMethod;
use std::collections::HashMap;

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}
