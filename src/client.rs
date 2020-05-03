use crate::message::{EgressMessage, IngressMessage};
use std::sync::mpsc::Sender;
use uuid::Uuid;

pub struct Client {
    id: Uuid,
    sender: Sender<EgressMessage>,
}

impl Client {
    pub fn new(id: Uuid, sender: Sender<EgressMessage>) -> Self {
        Client { id, sender }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn on_message(&self, message: &IngressMessage) {
        println!("on_message: {:?}", message.message);
    }
}
