use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::web::Data;
pub use lazy_static::lazy_static;
use domain::user::ports::primary::{CreateUserPort, GetUserPort};
use domain::user::ports::secondary::UserDataAccessPort;
use domain::user::use_case::{CreateUserUseCase, GetUserUseCase};
use postgresql::user::adapter::UserDataAccessAdapter;
pub use rest_server::user::controller::user_routes::user_routes::{create, get_by_id, get_all};

lazy_static! {
    pub static ref CREATE_USER_PORT: Arc<dyn CreateUserPort> = {
        let _ = postgresql::config::database_config::POOL.get();

        Arc::new(CreateUserUseCase::new(USER_DATA_ACCESS_CONTAINER.clone()))
    };
}

lazy_static! {
    pub static ref GET_USER_PORT: Arc<dyn GetUserPort> = {
        let _ = postgresql::config::database_config::POOL.get();

        Arc::new(GetUserUseCase::new(USER_DATA_ACCESS_CONTAINER.clone()))
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
            .app_data(Data::new(CREATE_USER_PORT.clone()))
            .service(create)
            .app_data(Data::new(GET_USER_PORT.clone()))
            .service(get_by_id)
            .service(get_all)
    }).bind(("127.0.0.1", 8080))
        ?.run()
        .await
}