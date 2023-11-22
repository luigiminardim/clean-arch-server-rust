use std::sync::Mutex;

type ThreadSafeNameGateway = Mutex<dyn NameGateway + Send + Sync>;

pub trait GreetUsecase {
    fn exec(&self) -> String;
}

pub trait NameGateway {
    fn get_name(&self) -> String;
}

pub struct GreetUsecaseImpl {
    name_gateway: Box<ThreadSafeNameGateway>, // HERE YOU CAN CHANGE THE WRAPPER (Box, Rc, Arc, none, etc)
}

impl GreetUsecaseImpl {
    pub fn new(name_gateway: Box<ThreadSafeNameGateway>) -> Self {
        Self { name_gateway }
    }
}

impl GreetUsecase for GreetUsecaseImpl {
    fn exec(&self) -> String {
        // NOTE: you probably wanna implement an error type or use something like `anyhow`
        let name = self.name_gateway.lock().unwrap().get_name();
        format!("Hello, {}!", name)
    }
}
