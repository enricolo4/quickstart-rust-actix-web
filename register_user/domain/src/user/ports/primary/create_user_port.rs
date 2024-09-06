use crate::user::model::{User, UserToCreate};

pub trait CreateUserPort: Sync + Send {
    fn create(&self, user_to_create: UserToCreate) -> User;
}