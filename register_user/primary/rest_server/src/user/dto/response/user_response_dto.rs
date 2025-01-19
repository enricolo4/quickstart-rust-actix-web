use serde::{Deserialize, Serialize};
use domain::user::model::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDTO {
    pub id: String,
    pub name: String,
    pub email: String
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UsersResponseDTO {
    count: u32,
    users: Vec<UserResponseDTO>
}

impl UserResponseDTO {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

impl UsersResponseDTO {
    pub fn new(
        count: u32,
        users: Vec<UserResponseDTO>
    ) -> Self {
        Self { count, users }
    }
}


pub trait UserToResponseDTO {
    fn to_user_response_dto(&self) -> UserResponseDTO;
}

pub trait UsersToResponseDTO {
    fn to_users_response_dto(&self) -> UsersResponseDTO;
}

impl UserToResponseDTO for User  {
    fn to_user_response_dto(&self) -> UserResponseDTO {
        UserResponseDTO::new (
            self.id.to_string(),
            self.name.to_string(),
            self.email.to_string()
        )
    }
}

impl UsersToResponseDTO for Vec<User> {
    fn to_users_response_dto(&self) -> UsersResponseDTO {
        UsersResponseDTO {
            count: self.len() as u32,
            users: self.iter().map(|user| user.to_user_response_dto()).collect()
        }
    }
}

impl UsersToResponseDTO for User {
    fn to_users_response_dto(&self) -> UsersResponseDTO {
        UsersResponseDTO {
            count: 1,
            users: vec![self.to_user_response_dto()]
        }
    }
}
