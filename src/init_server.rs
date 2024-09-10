use actix_web::{App, HttpServer};
use actix_web::web::Data;
use register_user::{create_user, USER_CONTROLLER_CONTAINER};

#[actix_web::main]
pub async fn init_rest_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(USER_CONTROLLER_CONTAINER.clone()))
            .service(create_user)
    }).bind(("127.0.0.1", 8080))
        ?.run()
        .await
}