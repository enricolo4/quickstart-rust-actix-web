#[derive(Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String
}

impl User {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

#[derive(Debug)]
pub struct UserToCreate {
    pub name: String,
    pub email: String
}

impl UserToCreate {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }
}