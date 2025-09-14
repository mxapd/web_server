use crate::actors::messages::DatabaseMessage;

pub struct PostgresActor {
    client: postgres::Client,
    receiver: std::sync::mpsc::Receiver<DatabaseMessage>,
}

impl PostgresActor {
    pub fn new(
        client: postgres::Client,
        receiver: std::sync::mpsc::Receiver<DatabaseMessage>,
    ) -> Self {
        Self { client, receiver }
    }

    pub fn run(self) {}
}
