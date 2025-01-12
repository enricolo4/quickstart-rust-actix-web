use std::sync::Arc;
use crate::user::model::{User, UserToCreate};
use crate::user::ports::primary::CreateUserPort;
use crate::user::ports::secondary::UserDataAccessPort;

pub struct CreateUserUseCase {
    user_data_access_port: Arc<dyn UserDataAccessPort>
}

impl CreateUserUseCase {
    pub fn new(user_data_access_port: Arc<dyn UserDataAccessPort>) -> Self {
        Self { user_data_access_port }
    }
}

impl CreateUserPort for CreateUserUseCase  {
    fn create(&self, user_to_create: UserToCreate) -> User {
        self.user_data_access_port.save(user_to_create)
    }
}