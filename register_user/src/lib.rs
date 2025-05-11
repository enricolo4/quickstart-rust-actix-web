use domain::user::ports::secondary::UserDataAccessPort;
use domain::user::use_case::{CreateUserUseCase, GetUserUseCase};
use postgresql::user::adapter::UserDataAccessAdapter;
pub use lazy_static::lazy_static;
use std::sync::Arc;
use domain::user::ports::primary::{CreateUserPort, GetUserPort};

lazy_static! {
    pub static ref CREATE_USER_PORT: Arc<dyn CreateUserPort> = {
        Arc::new(CreateUserUseCase::new(USER_DATA_ACCESS_CONTAINER.clone()))
    };
}

lazy_static! {
    pub static ref GET_USER_PORT: Arc<dyn GetUserPort> = {
        Arc::new(GetUserUseCase::new(USER_DATA_ACCESS_CONTAINER.clone()))
    };
}

lazy_static! {
    pub static ref USER_DATA_ACCESS_CONTAINER: Arc<dyn UserDataAccessPort> = {
        let _ = postgresql::config::database_config::POOL.get();

        Arc::new(UserDataAccessAdapter::new())
    };
}