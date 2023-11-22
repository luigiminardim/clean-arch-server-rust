use std::sync::{Arc, Mutex};

use crate::in_memory_name_gateway::NameGateway;

type ArcNameGateway = Arc<Mutex<dyn NameGateway + Send + Sync>>;

pub trait GreetUsecase {
    fn exec(&self) -> String;
}

pub struct GreetUsecaseImpl {
    name_gateway: ArcNameGateway, // HERE YOU CAN CHANGE THE WRAPPER (Box, Rc, Arc, none, etc)
}

impl GreetUsecaseImpl {
    pub fn new(name_gateway: ArcNameGateway) -> Self {
        Self { name_gateway }
    }
}

impl GreetUsecase for GreetUsecaseImpl {
    fn exec(&self) -> String {
        let name = self.name_gateway.lock().unwrap().get_name();
        format!("Hello, {}!", name)
    }
}
