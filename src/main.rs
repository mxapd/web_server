use std::io::{Read, Result};
use std::net::{TcpListener, TcpStream};
use std::string;

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

    let _ = parse_request(&buf);

    if let Ok(text) = String::from_utf8(buf.clone()) {
        //    println!("Message: {}", text);
    }

    Ok(())
}

fn parse_request(buffer: &Vec<u8>) -> Result<()> {
    let buffer_string = String::from_utf8_lossy(buffer);

    println!("{}", buffer_string);

    let (header_part, body_part) = buffer_string.split_once("\r\n\r\n").unwrap();

    println!(
        "header part: \n{}\n{}\n{}\n\n",
        "-".repeat(50),
        header_part,
        "-".repeat(50)
    );

    println!(
        "body part: \n{}\n{}\n{}\n\n",
        "-".repeat(50),
        body_part,
        "-".repeat(50)
    );

    // save body as raw bytes
    // parse through the header to get:
    // method path version from the first line
    // headers every line until body separator

    let header_parts = header_part
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let http_method: HttpMethod;

    match header_parts[0].as_str() {
        "GET" => http_method = HttpMethod::GET,
        "POST" => http_method = HttpMethod::POST,
        _ => println!("no match"),
    }

    println!("METHOD: {}", &header_parts[0]);
    //    let http_request: HttpRequest = {
    //        method:
    //        path: String,
    //        version: String,
    //        headers: HashMap<String, String>,
    //        body: Vec<u8>,
    //    };

    Ok(())
}
