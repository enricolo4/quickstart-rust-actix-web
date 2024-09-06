use std::sync::Arc;
use lazy_static::lazy_static;
pub use rest_server::user::controller::CreateUseController;
use domain::user::use_case::CreateUserUseCase;

lazy_static! {
    pub static ref USER_CONTROLLER_CONTAINER: Arc<CreateUseController> = {
        let _create_user_use_case = CreateUserUseCase::new();
        Arc::new(CreateUseController::new(Arc::new(_create_user_use_case)))
    };
}