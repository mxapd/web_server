mod actors;
mod html;
mod http;
mod server;

fn main() {
    server::start_server("127.0.0.1:2323").unwrap();
}
