use std::sync::mpsc;

pub enum RequestMessage {
    HandleHttp {
        stream: std::net::TcpStream,
        response: mpsc::Sender::Sender<Result(), String>>
    },
    Shutdown,
}
