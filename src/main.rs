use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use register_user::{CreateUseController, USER_CONTROLLER_CONTAINER};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(USER_CONTROLLER_CONTAINER.clone()))
            .route("/users", web::post().to(CreateUseController::create))
    }).bind(("127.0.0.1", 8080))
        ?.run()
        .await
}