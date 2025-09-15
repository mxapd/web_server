use std::sync::mpsc::Sender;

use crate::actors::messages::{DatabaseMessage, HandlerMessage, RouterMessage};

pub struct ActorDirectory {
    pub database: Sender<DatabaseMessage>,
    pub router: Sender<RouterMessage>,
    pub handler: Sender<HandlerMessage>,
}

impl ActorDirectory {
    pub fn new(
        database: Sender<DatabaseMessage>,
        router: Sender<RouterMessage>,
        handler: Sender<HandlerMessage>,
    ) -> Self {
        Self {
            database,
            router,
            handler,
        }
    }
}
