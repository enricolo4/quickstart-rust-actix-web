use serde::{Deserialize, Serialize};
use domain::user::model::{User, UserToCreate};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToCreateRequestDTO {
    name: String,
    email: String
}

impl UserToCreateRequestDTO {
    pub fn new(name: String, email: String) -> Self {
        UserToCreateRequestDTO { name, email }
    }

    pub fn to_model(&self) -> UserToCreate {
        UserToCreate::new(
            self.name.to_string(),
            self.email.to_string()
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDTO {
    id: String,
    name: String,
    email: String
}

impl UserResponseDTO {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

pub trait UserToResponseDTO {
    fn to_dto(&self) -> UserResponseDTO;
}

impl UserToResponseDTO for User  {
    fn to_dto(&self) -> UserResponseDTO {
        UserResponseDTO::new (
            self.id.to_string(),
            self.name.to_string(),
            self.email.to_string()
        )
    }
}