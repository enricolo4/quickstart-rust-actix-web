#[actix_web::scope("/users")]
pub mod user_routes {
    use actix_web::{get, post, Responder, HttpResponse};
    use crate::user::controller::user_controller::UserController;
    use std::sync::Arc;
    use actix_web::web::{Data, Json, Path};
    use uuid::Uuid;
    use crate::user::dto::UserToCreateRequestDTO;

    #[post("")]
    pub async fn create(user_controller: Data<Arc<UserController>>, user_to_create_request_dto: Json<UserToCreateRequestDTO>) -> impl Responder {
        let _response = user_controller.create_new_user(user_to_create_request_dto);
        Json(_response)
    }

    #[get("/{id}")]
    pub async fn get_by_id(user_controller: Data<Arc<UserController>>, id: Path<String>) -> impl Responder {
        let _id = Uuid::parse_str(id.into_inner().as_str()).unwrap();
        let _response = user_controller.get_by_id(_id);
        Json(_response)
    }

    #[get("")]
    pub async fn get_all(user_controller: Data<Arc<UserController>>) -> HttpResponse {
        HttpResponse::Ok().json(user_controller.get_all())
    }
}