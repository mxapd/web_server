use crate::actors::postgres_actor::PostgresActor;
use crate::actors::request_actor::RequestActor;

use postgres::NoTls;
use std::net::TcpListener;
use std::sync::mpsc;
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

    for stream in listener.incoming() {
        let db_tx_clone = db_tx.clone();
        thread::spawn(move || {
            let actor = RequestActor::new(stream.unwrap(), db_tx_clone);
            actor.run();
        });
    }

    Ok(())
}
