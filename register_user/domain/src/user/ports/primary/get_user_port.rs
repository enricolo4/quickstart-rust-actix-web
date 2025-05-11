use crate::user::model::User;

pub trait GetUserPort: Sync + Send {
    fn get_by_id(&self, id: i64) -> Option<User>;
    fn get_all(&self) -> Vec<User>;
}