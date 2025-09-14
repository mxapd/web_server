mod database_actor;
mod html;
mod http;
mod messages;
mod request_actor;

use database_actor::PostgresActor;
use postgres::NoTls;
use request_actor::RequestActor;
use std::io::Result;
use std::net::TcpListener;
use std::thread;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:2323")?;

    for stream in listener.incoming() {
        thread::spawn(move || {
            let client = postgres::Client::connect(
                "host=localhost user=bruv password=n0t@pAssWornk dbname=mydatabase",
                NoTls,
            );

            let db_actor = PostgresActor::new(client.unwrap());

            let actor = RequestActor::new(stream.unwrap(), db_actor);
            actor.run().unwrap();
        });
    }

    Ok(())
}
