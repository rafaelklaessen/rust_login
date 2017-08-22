use super::schema::users;

#[derive(Queryable, RustcEncodable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
}

impl User {
    pub fn to_owned(&self) -> User {
        User {
            id: self.id,
            username: self.username.to_owned(),
            email: self.email.to_owned(),
            name: self.name.to_owned(),
            password: self.password.to_owned()
        }
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub name: String,
    pub password: String,
}
