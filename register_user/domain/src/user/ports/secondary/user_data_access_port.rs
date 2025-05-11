use crate::user::model::{User, UserToCreate};

pub trait UserDataAccessPort: Sync + Send {
    fn save(&self, user_to_create: UserToCreate) -> User;
    fn find_by_id(&self, id: i64) -> Option<User>;
    fn find_all(&self) -> Vec<User>;
}