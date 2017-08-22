use regex::Regex;
use diesel::pg::PgConnection;
use ::users;

pub fn unused_username(conn: &PgConnection, username: &String) -> bool {
    let existing_user = users::get(conn, username);
    existing_user.is_none()
}

pub fn valid_email(email: &String) -> bool {
    let email_regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4}$").unwrap();
    email_regex.is_match(&email)
}

pub fn valid_password(password: &String) -> bool {
    password.len() >= 5
}
