use crate::greet_usecase::NameGateway;

pub struct InMememoryNameGateway {}

impl NameGateway for InMememoryNameGateway {
    fn get_name(&self) -> String {
        "João".to_string()
    }
}
