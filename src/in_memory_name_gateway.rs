pub trait NameGateway {
    fn get_name(&self) -> String;
}

pub struct InMememoryNameGateway {}

impl NameGateway for InMememoryNameGateway {
    fn get_name(&self) -> String {
        "João".to_string()
    }
}
