pub use crate::user::controller::user_controller::CreateUseController;

mod user_controller;

#[actix_web::scope("/users")]
pub mod user_routes {
    use actix_web::{post, Responder};
    use crate::user::controller::user_controller::CreateUseController;
    use std::sync::Arc;
    use actix_web::web::{Data, Json};
    use crate::user::dto::UserToCreateRequestDTO;

    #[post("")]
    pub async fn create_user(create_user_controller: Data<Arc<CreateUseController>>, user_to_create_request_dto: Json<UserToCreateRequestDTO>) -> impl Responder {
        let _response = create_user_controller.create_new_user(user_to_create_request_dto);
        Json(_response)
    }
}
