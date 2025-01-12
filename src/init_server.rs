use actix_web::{App, HttpServer};
use actix_web::web::Data;
use register_user::{create, get_all, get_by_id, USER_CONTROLLER_CONTAINER};

#[actix_web::main]
pub async fn init_rest_server() -> std::io::Result<()> {
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