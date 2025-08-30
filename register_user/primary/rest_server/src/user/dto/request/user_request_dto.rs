use serde::{Deserialize, Serialize};
use domain::user::model::UserToCreate;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToCreateRequestDTO {
    name: String,
    email: String,
    cpf: String,
}

impl UserToCreateRequestDTO {
    pub fn new(name: String, email: String, cpf: String) -> Self {
        UserToCreateRequestDTO { name, email, cpf }
    }

    pub fn to_model(&self) -> UserToCreate {
        UserToCreate::new(
            self.name.to_string(),
            self.email.to_string(),
            self.cpf.to_string()
        )
    }
}
