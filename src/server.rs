use crate::actors::actor_directory::ActorDirectory;
use crate::actors::postgres_actor::PostgresActor;
use crate::actors::request_actor::RequestActor;
use crate::actors::router_actor::RouterActor;

use postgres::NoTls;
use std::net::TcpListener;
use std::sync::{Arc, mpsc};
use std::thread;

pub fn start_server(host: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(host)?;

    let client = postgres::Client::connect(
        "host=localhost user=bruv password=n0t@pAssWornk dbname=mydatabase",
        NoTls,
    )
    .unwrap();
    let (db_tx, db_rx) = mpsc::channel();
    let db_actor = PostgresActor::new(client, db_rx);
    thread::spawn(move || db_actor.run());

    let (router_tx, router_rx) = mpsc::channel();
    let router = RouterActor::new(router_rx);
    thread::spawn(move || router.run());

    let actor_directory = Arc::new(ActorDirectory::new(db_tx, router_tx));

    for stream in listener.incoming() {
        let actor_directory_reference = Arc::clone(&actor_directory);
        thread::spawn(move || {
            let actor = RequestActor::new(stream.unwrap(), actor_directory_reference);
            actor.run();
        });
    }

    Ok(())
}
