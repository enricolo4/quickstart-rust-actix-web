use uuid::Uuid;
use domain::user::model::{User, UserToCreate};
use domain::user::ports::secondary::UserDataAccessPort;

struct UserAdapter {
    
}

impl UserDataAccessPort for UserAdapter {
    fn save(user_to_create: UserToCreate) -> User {
        todo!()
    }

    fn find_by_id(id: Uuid) -> Option<User> {
        todo!()
    }
}