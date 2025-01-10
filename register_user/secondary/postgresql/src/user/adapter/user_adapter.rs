use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::dsl::insert_into;
use uuid::Uuid;
use domain::user::model::{User, UserToCreate};
use domain::user::ports::secondary::UserDataAccessPort;
use crate::config::database_config::get_connection;
use crate::user::dbo::{UserDBO, UserToCreateToDBO};
use crate::user::schema::schema::users::dsl::users;

struct UserAdapter;

impl UserDataAccessPort for UserAdapter {
    fn save(&self, user_to_create: UserToCreate) -> User {

        insert_into(users)
            .values(&user_to_create.to_dbo())
            .returning(UserDBO::as_returning())
            .get_result(&mut get_connection())
            .unwrap().to_model()
    }

    fn find_by_id(&self, id: Uuid) -> Option<User> {
        users
            .find(id)
            .select(UserDBO::as_select())
            .get_result::<UserDBO>(&mut get_connection())
            .map_or(None, |user_dbo| Some(user_dbo.to_model()))
    }
}

