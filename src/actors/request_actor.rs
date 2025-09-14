use crate::actors::actor_directory::ActorDirectory;
use crate::actors::messages::RouterMessage;
use crate::http::http_method::HttpMethod;
use crate::http::http_request::HttpRequest;
use crate::http::http_response::HttpResponse;

use std::collections::HashMap;
use std::error::Error;
use std::io::{BufReader, prelude::*};
use std::net::TcpStream;
use std::sync::{Arc, mpsc};

pub struct RequestActor {
    stream: TcpStream,
    actor_directory: Arc<ActorDirectory>,
}

impl RequestActor {
    pub fn new(stream: TcpStream, actor_directory: Arc<ActorDirectory>) -> Self {
        Self {
            stream,
            actor_directory,
        }
    }

    pub fn run(self) {
        Self::handle_client(self.stream, self.actor_directory).unwrap();
    }

    fn handle_client(
        stream: TcpStream,
        actor_directory: Arc<ActorDirectory>,
    ) -> Result<(), Box<dyn Error>> {
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

        let http_request = Self::parse_request(&request_data)?;

        let (response_tx, response_rx): (mpsc::Sender<HttpResponse>, mpsc::Receiver<HttpResponse>) =
            mpsc::channel();

        actor_directory.router.send(RouterMessage::Route {
            http_request,
            response_tx,
        })?;

        let response = response_rx.recv()?;

        Self::send_response(stream, response)?;

        Ok(())
    }

    fn parse_request(buffer: &Vec<u8>) -> Result<HttpRequest, Box<dyn Error>> {
        let (header_data, body_data) = Self::split_headers_body(&buffer.as_slice());

        let body_data = body_data.map(|b| b.to_vec());

        let request_line = Self::parse_request_line(header_data);

        let http_headers = Self::parse_headers(header_data);

        let http_request: HttpRequest = HttpRequest {
            method: request_line.0,
            path: request_line.1.clone(),
            version: request_line.2.clone(),
            headers: http_headers,
            body: body_data,
        };

        Ok(http_request)
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
}
