use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use rustc_serialize::json;
use std::collections::HashMap;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

#[derive(RustcEncodable)]
pub struct Success {
    pub success: bool,
}

#[derive(RustcEncodable)]
pub struct Error {
    pub error_type: String,
    pub error_description: String,
}

pub fn success() -> (status::Status, Header<ContentType>, String) {
    let success = Success { success: true };
    let success = json::encode(&success).unwrap();
    json(success)
}

pub fn error(error_type: &str, error_msg: &str) -> (status::Status, Header<ContentType>, String) {
    let error = Error {
        error_type: error_type.to_owned(),
        error_description: error_msg.to_string()
    };
    let error = json::encode(&error).unwrap();
    json_with_status(status::BadRequest, error)
}

pub fn json_with_status(status: status::Status, json: String) -> (status::Status, Header<ContentType>, String) {
    (status, Header(ContentType::json()), json)
}

pub fn json(json: String) -> (status::Status, Header<ContentType>, String) {
    json_with_status(status::Ok, json)
}

pub fn form_field(form: &HashMap<String, Vec<String>>, field: &str) -> Option<String> {
    match form.get(field) {
        Some(value) => {
            let val = value[0].to_owned();
            if val.trim().is_empty() {
                return None;
            } else {
                return Some(val);
            }
        },
        None => None
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
