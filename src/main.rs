use std::collections::HashMap;
use std::io::{Read, Result};
use std::net::{TcpListener, TcpStream};

mod http;
use http::{HttpMethod, HttpRequest, HttpStatus};

fn main() -> Result<()> {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:2323")?;

    for stream in listener.incoming() {
        let _ = handle_client(stream?);
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    println!("Client connected: {:?}", stream.peer_addr()?);

    let mut buf: Vec<u8> = Vec::new();
    stream.read_to_end(&mut buf)?;

    //println!("Recieved {} bytes from client", buf.len());

    let http_request = parse_request(&buf)?;

    // match http_request.path {
    //     "/" =>
    // }

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

fn parse_request_line();
fn parse_headers();

fn parse_request(buffer: &Vec<u8>) -> Result<HttpRequest> {
    // TODO: revamp parsing, rn its splitting twice once using bytes and once after turning it into
    // a string. Would like byte split to get headers, body. then turn headers into string or vec
    // to continue parsing

    // TODO: Split into smaller functions

    // TODO: Error handling, returning errors. Create custom error HttpParseError

    dbg!(&body_data);

    let buffer_string = String::from_utf8_lossy(buffer);

    println!("{}", buffer_string);

    let (header_part, body_part) = buffer_string.split_once("\r\n\r\n").unwrap();

    let header_parts = header_part
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let http_method: HttpMethod;

    // GET METHOD PART
    match header_parts[0].as_str() {
        "GET" => http_method = HttpMethod::GET,
        "POST" => http_method = HttpMethod::POST,
        _ => panic!("no match on http method"),
    }

    let http_path: &String = &header_parts[1];
    let http_version: &String = &header_parts[2];

    // loop trough the rest and map header keys and values

    let mut http_headers: HashMap<String, String> = HashMap::new();

    let header_rows = header_part.split('\n');

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

    let http_request: HttpRequest = HttpRequest {
        method: http_method,
        path: http_path.clone(),
        version: http_version.clone(),
        headers: http_headers,
        body: body_data,
    };

    Ok(http_request)
}
