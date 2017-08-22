use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use models::{User, NewUser};
use schema::users;
use schema::users::dsl::*;
use bcrypt::{DEFAULT_COST, hash};

pub mod session;
pub mod api;
pub mod validations;

pub fn create(conn: &PgConnection, u_username: &String, u_email: String, u_name: String, u_password: String) -> Result<User, Error> {
    let new_user = NewUser {
        username: u_username.clone().to_owned(),
        email: u_email,
        name: u_name,
        password: hash(&u_password, DEFAULT_COST).unwrap()
    };

    diesel::insert(&new_user)
        .into(users::table)
        .get_result(conn)
}

pub fn update(conn: &PgConnection, old_user: User, u_username: String, u_email: String, u_name: String, u_password: String) -> Result<User, Error> {
    let mut new_password = u_password;
    if new_password.is_empty() {
        new_password = old_user.password;
    } else {
        new_password = hash(&new_password, DEFAULT_COST).unwrap();
    }

    diesel::update(users.filter(username.eq(old_user.username)))
        .set((
            username.eq(u_username),
            email.eq(u_email),
            name.eq(u_name),
            password.eq(new_password),
        ))
        .get_result::<User>(conn)
}

pub fn delete(conn: &PgConnection, u_username: String) -> Result<User, Error> {
    diesel::delete(users.filter(username.eq(u_username)))
        .get_result::<User>(conn)
}

pub fn get(conn: &PgConnection, u_username: &String) -> Option<User> {
    let db_user = users.filter(username.eq(u_username))
        .limit(1)
        .load::<User>(conn)
        .expect("Error loading user");

    let db_user = db_user.get(0);

    match db_user {
        Some(user) => Some(user.to_owned()),
        None => None
    }
}
