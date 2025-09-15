mod actors;
mod handlers;
mod html;
mod http;
mod server;

//TODO: Go through code and add comments
//TODO: Dont create a new thread for each connection, maybe create a threadpool or just use one
//      TCP/request actor with async
//TODO: Custom errors
//TODO: Reduce or remove all blocking actions by using async
//TODO: Add timeouts to stop connections from lasting forever
//TODO: Add logging and metrics
//
//TODO: handlers module. ex handlers::home_page() and more
//TODO: htmlbuilder

fn main() {
    server::start_server("127.0.0.1:2323").unwrap();
}
