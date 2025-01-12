use std::sync::Arc;
use actix_web::web::Json;
use uuid::Uuid;
use domain::user::ports::primary::{CreateUserPort, GetUserPort};
use crate::user::dto::{UserToCreateRequestDTO, UserResponseDTO, UsersResponseDTO, UserToResponseDTO, UsersToResponseDTO};

pub struct UserController {
    create_user_port: Arc<dyn CreateUserPort>,
    get_user_port: Arc<dyn GetUserPort>
}

impl UserController {
    pub fn new(
        create_user_port: Arc<dyn CreateUserPort>,
        get_user_port: Arc<dyn GetUserPort>
    ) -> Self {
            Self {
                create_user_port,
                get_user_port
            }
    }

    pub fn create_new_user(&self, user_to_create_request_dto: Json<UserToCreateRequestDTO>) -> UserResponseDTO {
        self.create_user_port.create(user_to_create_request_dto.0.to_model()).to_user_response_dto()
    }

    pub fn get_by_id(&self, id: Uuid) -> UsersResponseDTO {
        match self.get_user_port.get_by_id(id) {
            Some(user) => user.to_users_response_dto(),
            None => UsersResponseDTO::default()
        }
    }

    pub fn get_all(&self) -> UsersResponseDTO {
        self.get_user_port.get_all().to_users_response_dto()
    }
}

