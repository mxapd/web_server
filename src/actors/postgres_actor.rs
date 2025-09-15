use crate::actors::messages::DatabaseMessage;

pub struct PostgresActor {
    client: postgres::Client,
    mailbox: std::sync::mpsc::Receiver<DatabaseMessage>,
}

impl PostgresActor {
    pub fn new(
        client: postgres::Client,
        mailbox: std::sync::mpsc::Receiver<DatabaseMessage>,
    ) -> Self {
        Self { client, mailbox }
    }

    pub fn run(mut self) {
        for message in self.mailbox {
            match message {
                DatabaseMessage::Query { sql, response_tx } => {
                    let result = self.client.query(&sql, &[]);
                    let _ = response_tx.send(result);
                }
                _ => println!("PostgresActor: unknown message received"),
            }
        }
    }
}
