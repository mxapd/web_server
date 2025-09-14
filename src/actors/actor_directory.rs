use std::sync::mpsc::Sender;

use crate::actors::messages::{DatabaseMessage, RouterMessage};

pub struct ActorDirectory {
    pub database: Sender<DatabaseMessage>,
    pub router: Sender<RouterMessage>,
}

impl ActorDirectory {
    pub fn new(database: Sender<DatabaseMessage>, router: Sender<RouterMessage>) -> Self {
        Self { database, router }
    }
}
