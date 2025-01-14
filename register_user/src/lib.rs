use std::sync::Arc;
pub use lazy_static::lazy_static;
use rest_server::user::controller::UserController;
use domain::user::use_case::{CreateUserUseCase, GetUserUseCase};
use postgresql::user::adapter::UserDataAccessAdapter;
pub use rest_server::user::controller::user_routes::user_routes::{create, get_by_id, get_all};

lazy_static! {
    pub static ref USER_CONTROLLER_CONTAINER: Arc<UserController> = {
        let _ = postgresql::config::database_config::POOL.get();

        let _user_data_access_port = Arc::new(UserDataAccessAdapter::new());
        let _create_user_use_case = Arc::new(CreateUserUseCase::new(_user_data_access_port.clone()));
        let _get_user_use_case = Arc::new(GetUserUseCase::new(_user_data_access_port.clone()));

        Arc::new(UserController::new(_create_user_use_case.clone(), _get_user_use_case.clone()))
    };
}