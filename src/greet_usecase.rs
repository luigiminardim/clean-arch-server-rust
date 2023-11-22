pub trait GreetUsecase {
    fn exec(&self) -> String;
}

pub trait NameGateway {
    fn get_name(&self) -> String;
}

pub struct GreetUsecaseImpl {
    name_gateway: Box<dyn NameGateway>, // HERE YOU CAN CHANGE THE WRAPPER (Box, Rc, Arc, none, etc)
}

impl GreetUsecase for GreetUsecaseImpl {
    fn exec(&self) -> String {
        let name = self.name_gateway.get_name();
        format!("Hello, {}!", name)
    }
}
