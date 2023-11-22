mod greet_usecase;
mod in_memory_name_gateway;

use greet_usecase::{GreetUsecase, GreetUsecaseImpl};
use rocket as _rocket;

struct State {
    greet_usecase: Box<dyn GreetUsecase + Send + Sync>,
}

#[_rocket::get("/")]
fn index(state: &_rocket::State<State>) -> String {
    state.greet_usecase.exec()
}

#[_rocket::launch]
fn rocket() -> _ {
    let name_gateway = Box::new(in_memory_name_gateway::InMememoryNameGateway {});
    let greet_usecase = Box::new(GreetUsecaseImpl::new(name_gateway));
    let state = State { greet_usecase };
    rocket::build()
        .manage(state)
        .mount("/", _rocket::routes![index])
}
