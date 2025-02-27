use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
pub use lazy_static::lazy_static;
use domain::user::ports::secondary::UserDataAccessPort;
use rest_server::user::controller::UserController;
use domain::user::use_case::{CreateUserUseCase, GetUserUseCase};
use postgresql::user::adapter::UserDataAccessAdapter;
pub use rest_server::user::controller::user_routes::user_routes::{create, get_by_id, get_all};

lazy_static! {
    pub static ref USER_CONTROLLER_CONTAINER: Arc<UserController> = {
        let _ = postgresql::config::database_config::POOL.get();

        let _create_user_use_case = Arc::new(CreateUserUseCase::new(USER_DATA_ACCESS_CONTAINER.clone()));
        let _get_user_use_case = Arc::new(GetUserUseCase::new(USER_DATA_ACCESS_CONTAINER.clone()));

        Arc::new(UserController::new(_create_user_use_case.clone(), _get_user_use_case.clone()))
    };
}

lazy_static! {
    pub static ref USER_DATA_ACCESS_CONTAINER: Arc<dyn UserDataAccessPort> = {
        let _ = postgresql::config::database_config::POOL.get();

        Arc::new(UserDataAccessAdapter::new())
    };
}

#[actix_web::main]
pub async fn init_user_rest_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(USER_CONTROLLER_CONTAINER.clone()))
            .service(create)
            .service(get_by_id)
            .service(get_all)
    }).bind(("127.0.0.1", 8080))
        ?.run()
        .await
}