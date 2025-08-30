use diesel::{Insertable, Queryable, Selectable};
use diesel::internal::derives::multiconnection::chrono::{DateTime, Utc};
use domain::user::model::{User, UserToCreate};
use crate::user::schema::schema::users;
use tsid::create_tsid;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDBO {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub cpf: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>
}

impl UserDBO {
    pub fn new(
        id: i64,
        name: String,
        email: String,
        cpf: String,
        created_at: DateTime<Utc>,
        modified_at: Option<DateTime<Utc>>
    ) -> Self { Self { id, name, email, cpf, created_at, modified_at }
    }

    pub fn to_model(&self) -> User {
        User::new(
            self.id as u64,
            self.name.to_string(),
            self.email.to_string(),
            self.cpf.to_string()
        )
    }
}

pub trait UserToCreateToDBO {
    fn to_dbo(&self) -> UserDBO;
}

impl UserToCreateToDBO for UserToCreate {
    fn to_dbo(&self) -> UserDBO {
        UserDBO::new(
            create_tsid().number() as i64,
            self.name.to_string(),
            self.email.to_string(),
            self.cpf.to_string(),
            Utc::now(),
            Some(Utc::now())
        )
    }
}