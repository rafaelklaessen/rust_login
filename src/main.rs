extern crate iron;
extern crate iron_sessionstorage;
extern crate router;
extern crate staticfile;
extern crate mount;
#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate diesel_codegen;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use router::Router;
use staticfile::Static;
use mount::Mount;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use iron_sessionstorage::traits::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

pub mod schema;
pub mod models;
pub mod users;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn index(_req: &mut Request) -> IronResult<Response> {
    let mut file = File::open("public/index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Ok(Response::with((status::Ok, Header(ContentType::html()), contents)))
}

struct Username(String);

impl iron_sessionstorage::Value for Username {
    fn get_key() -> &'static str { "username" }
    fn into_raw(self) -> String { self.0 }
    fn from_raw(value: String) -> Option<Self> {
        Some(Username(value))
    }
}

fn get_session(req: &mut Request) -> IronResult<Response> {
    let session = match try!(req.session().get::<Username>()) {
        Some(username) => username.0,
        None => String::from("No session")
    };

    Ok(Response::with((status::Ok, session)))
}

fn set_session(req: &mut Request) -> IronResult<Response> {
    try!(req.session().set(Username(String::from("le epic"))));
    Ok(Response::with((status::Ok, "set")))
}

fn main() {
    let mut router = Router::new();
    router.get("/", index, "index");
    router.get("/get_session", get_session, "get_session");
    router.get("/set_session", set_session, "set_session");

    let connection = establish_connection();
    let _user = users::create_user(&connection, "title", "kees", "Henk", "password");

    let my_secret = b"verysecret".to_vec();

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/public/", Static::new(Path::new("public/")));

    let mut ch = Chain::new(mount);
    ch.link_around(SessionStorage::new(SignedCookieBackend::new(my_secret)));
    let _res = Iron::new(ch).http("localhost:3000");
}
