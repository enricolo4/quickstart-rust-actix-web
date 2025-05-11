use std::sync::Arc;
use crate::user::model::User;
use crate::user::ports::primary::GetUserPort;
use crate::user::ports::secondary::UserDataAccessPort;

pub struct GetUserUseCase {
    user_data_access_port: Arc<dyn UserDataAccessPort>
}

impl GetUserUseCase {
    pub fn new(user_data_access_port: Arc<dyn UserDataAccessPort>) -> Self {
        Self { user_data_access_port }
    }
}

impl GetUserPort for GetUserUseCase {
    fn get_by_id(&self, id: i64) -> Option<User> {
        self.user_data_access_port.find_by_id(id)
    }

    fn get_all(&self) -> Vec<User> {
        self.user_data_access_port.find_all()
    }
}

