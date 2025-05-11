use actix_web::{App, HttpServer};
use actix_web::web::Data;
use register_user::{CREATE_USER_PORT, GET_USER_PORT};
pub use rest_server::user::controller::user_routes::user_routes::{create, get_by_id, get_all};

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

fn main() {
    init_user_rest_server().expect("Failed to initialize user REST server")
}