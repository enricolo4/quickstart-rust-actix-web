use uuid::Uuid;
use crate::user::model::{User, UserToCreate};

pub trait UserDataAccessPort: Sync + Send {
    fn save(user_to_create: UserToCreate) -> User;
    fn find_by_id(id: Uuid) -> Option<User>;
}