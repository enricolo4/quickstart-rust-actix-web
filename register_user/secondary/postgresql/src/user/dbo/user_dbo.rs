use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Insertable, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct UserDBO {
    id: Uuid,
    name: String,
    email: String
}