use super::schema::users;

#[derive(Queryable)]
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
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub name: &'a str,
    pub password: &'a str,
}
