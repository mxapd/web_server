pub struct PostgresActor {
    client: postgres::Client,
}

impl PostgresActor {
    pub fn new(client: postgres::Client) -> Self {
        Self { client }
    }

    pub fn run(self) {}
}
