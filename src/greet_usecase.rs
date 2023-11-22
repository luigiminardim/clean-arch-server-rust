pub trait GreetUsecase {
    fn exec(&self) -> String;
}

pub struct GreetUsecaseImpl {
    name: String,
}

impl GreetUsecaseImpl {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl GreetUsecase for GreetUsecaseImpl {
    fn exec(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}
