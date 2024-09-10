use uuid::Uuid;
use crate::user::model::{User, UserToCreate};
use crate::user::ports::primary::CreateUserPort;

impl CreateUserPort for CreateUserUseCase  {
    fn create(&self, user_to_create: UserToCreate) -> User {
        User::new(
            Uuid::new_v4(),
            user_to_create.name,
            user_to_create.email
        )
    }
}

impl CreateUserUseCase {
    pub fn new() -> Self { Self {} }
}

pub struct CreateUserUseCase {}