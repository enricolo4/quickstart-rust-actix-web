use diesel::{Insertable, Queryable, Selectable};
use diesel::internal::derives::multiconnection::chrono::{DateTime, Utc};
use uuid::Uuid;
use domain::user::model::{User, UserToCreate};
use crate::user::schema::schema::users;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDBO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>
}

impl UserDBO {
    pub fn new(
        id: Uuid,
        name: String,
        email: String,
        created_at: DateTime<Utc>,
        modified_at: Option<DateTime<Utc>>
    ) -> Self {
        Self { id, name, email, created_at, modified_at }
    }

    pub fn to_model(&self) -> User {
        User::new(
            self.id,
            self.name.to_string(),
            self.email.to_string()
        )
    }
}

pub trait UserToCreateToDBO {
    fn to_dbo(&self) -> UserDBO;
}

impl UserToCreateToDBO for UserToCreate {
    fn to_dbo(&self) -> UserDBO {
        UserDBO::new(
            Uuid::default(),
            self.name.to_string(),
            self.email.to_string(),
            Utc::now(),
            Some(Utc::now())
        )
    }
}