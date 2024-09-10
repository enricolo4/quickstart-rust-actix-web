use std::sync::Arc;
use actix_web::Responder;
use actix_web::web::{Data, Json};
use domain::user::ports::primary::CreateUserPort;
use crate::user::dto::{UserToCreateRequestDTO, UserToResponseDTO, UserResponseDTO};

pub struct CreateUseController {
    create_user_port: Arc<dyn CreateUserPort>
}

impl CreateUseController {
    pub fn new(create_user_port: Arc<dyn CreateUserPort>) -> Self { Self { create_user_port } }

    pub async fn create(create_user_controller: Data<Arc<CreateUseController>>, user_to_create_request_dto: Json<UserToCreateRequestDTO>) -> impl Responder  {
        let response = create_user_controller.create_new_user(user_to_create_request_dto);
        Json(response)
    }

    pub fn create_new_user(&self, user_to_create_request_dto: Json<UserToCreateRequestDTO>) -> UserResponseDTO {
        self.create_user_port.create(user_to_create_request_dto.0.to_model()).to_dto()
    }
}

