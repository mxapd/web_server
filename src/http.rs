use std::clone;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::result::Result;

// TODO: change read_to_end to read instantly somehow, maybe wrap in buffered reader or something
//       like that
// TODO: Error handling, returning errors. Create custom error HttpParseError
// TODO: Router function that looks at the route in the request and decides which static page to
//       serve

struct Html {
    content: String,
}

impl Html {
    fn from_file(filepath: String) -> Result<Html, Box<dyn Error>> {
        let file = File::open(filepath)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let html = Html { content: contents };

        Ok(html)
    }

    fn new() -> Html {
        Html {
            content: String::new(),
        }
    }

    fn into_bytes(self) -> Vec<u8> {
        self.content.into_bytes()
    }
}

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

pub fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Client connected: {:?}", stream.peer_addr()?);

    let mut buf: Vec<u8> = Vec::new();

    stream.read_to_end(&mut buf)?;

    println!("Recieved {} bytes from client", buf.len());

    let http_request = parse_request(&buf)?;

    match http_request.path.as_str() {
        "/" => {
            let html = Html::from_file(String::from("index.html"))?;
            let response = HttpResponse::from_html(html, HttpStatus::Ok);

            send_response(stream, response)?;
        }
        _ => panic!("route not found"),
    }

    Ok(())
}

fn send_response(mut stream: TcpStream, response: HttpResponse) -> Result<(), Box<dyn Error>> {
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

    stream.write_all(response_string.as_bytes())?;
    stream.write_all(&response.body)?;

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

    println!("{}", header_string);

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
