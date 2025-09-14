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

    pub fn run(self) {}
}
