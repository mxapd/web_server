use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, Write, prelude::*};
use std::net::TcpStream;
use std::result::Result;

use crate::html;

use html::Html;

// TODO: Error handling, returning errors. Create custom error HttpParseError
// TODO: Router function that looks at the route in the request and decides which static page to
//       serve
// TODO: Logging

struct HttpRequest {
    method: HttpMethod,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

#[derive(Clone)]
struct HttpResponse {
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    status_code: HttpStatus,
    // maybe no need since we can derive from status reason_phrase: String,
}

impl HttpResponse {
    fn from_html(html: Html, status: HttpStatus) -> Self {
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

    fn reason_phrase(&self) -> &str {
        match self.status_code {
            HttpStatus::Ok => "OK",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::InternalServerError => "Internal Server Error",
        }
    }
}

enum HttpMethod {
    GET,
    POST,
}

#[derive(Clone)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

pub fn handle_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Client connected: {:?}", stream.peer_addr()?);

    let mut reader = BufReader::new(&stream);
    let mut request_data = Vec::new();

    loop {
        let mut line = Vec::new();
        reader.read_until(b'\n', &mut line)?;
        request_data.extend_from_slice(&line);

        if line == b"\r\n" || line == b"\n" {
            break;
        }
    }

    println!("Received {} bytes from client", request_data.len());

    let http_request = parse_request(&request_data)?;

    match http_request.path.as_str() {
        "/" => {
            let html = Html::from_file("index.html".to_string());
            println!("loaded file result: {:?}", html);

            let html = html?; // only propagate after printing

            let response = HttpResponse::from_html(html, HttpStatus::Ok);
            println!("built response");

            send_response(stream, response)?;
        }
        "/nej" => {
            let html = Html::from_file("nej.html".to_string());
            println!("loaded file result: {:?}", html);

            let html = html?; // only propagate after printing

            let response = HttpResponse::from_html(html, HttpStatus::Ok);
            println!("built response");

            send_response(stream, response)?;
        }
        _ => panic!("route not found"),
    }

    Ok(())
}

fn send_response(mut stream: TcpStream, response: HttpResponse) -> Result<(), Box<dyn Error>> {
    println!("entered send response");

    let mut response_string = format!(
        "{} {} {}\r\n",
        response.version,
        response.status_code.clone() as u16,
        response.reason_phrase()
    );

    for (key, value) in &response.headers {
        response_string.push_str(format!("{}: {}\r\n", key, value).as_str());
    }

    response_string.push_str("Connection: close \r\n\r\n");

    println!("{}", response_string);

    stream.write_all(response_string.as_bytes())?;
    stream.write_all(&response.body)?;

    stream.flush()?;

    Ok(())
}

fn split_headers_body(buffer: &[u8]) -> (&[u8], Option<&[u8]>) {
    let separator: [u8; 4] = [0x0D, 0x0A, 0x0D, 0x0A];

    if let Some(pos) = buffer
        .windows(separator.len())
        .position(|window| window == separator)
    {
        (&buffer[..pos], Some(&buffer[pos + separator.len()..]))
    } else {
        (buffer, None)
    }
}

fn parse_request_line(header_data: &[u8]) -> (HttpMethod, String, String) {
    let header_string = String::from_utf8_lossy(header_data);

    println!("\nHeader:\n{}", header_string);

    let header_parts = header_string
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let http_method = match header_parts[0].as_str() {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        _ => panic!("no match on http method"),
    };

    let http_path: String = header_parts[1].clone();
    let http_version: String = header_parts[2].clone();

    (http_method, http_path, http_version)
}

fn parse_headers(header_data: &[u8]) -> HashMap<String, String> {
    let mut http_headers: HashMap<String, String> = HashMap::new();

    let header_string = String::from_utf8_lossy(header_data);

    let header_rows = header_string.split("\r\n");

    let mut i = 0;
    for row in header_rows {
        i += 1;

        if i < 2 {
            continue;
        }

        let (key, value) = row.split_once(':').unwrap();
        let value = value.trim_start();

        http_headers.insert(String::from(key), String::from(value));
    }

    http_headers
}

fn parse_request(buffer: &Vec<u8>) -> Result<HttpRequest, Box<dyn Error>> {
    let (header_data, body_data) = split_headers_body(&buffer.as_slice());

    let body_data = body_data.map(|b| b.to_vec());

    let request_line = parse_request_line(header_data);

    let http_headers = parse_headers(header_data);

    let http_request: HttpRequest = HttpRequest {
        method: request_line.0,
        path: request_line.1.clone(),
        version: request_line.2.clone(),
        headers: http_headers,
        body: body_data,
    };

    Ok(http_request)
}
