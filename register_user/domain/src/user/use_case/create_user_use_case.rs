use crate::user::model::{User, UserToCreate};
use crate::user::ports::primary::CreateUserPort;

impl CreateUserPort for CreateUserUseCase  {
    fn create(&self, user_to_create: UserToCreate) -> User {
        User::new(
            "test".to_string(),
            user_to_create.name,
            user_to_create.email
        )
    }
}

impl CreateUserUseCase {
    pub fn new() -> Self { Self {} }
}

pub struct CreateUserUseCase {}