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

use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

pub mod schema;
pub mod models;
pub mod users;
pub mod session;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn index(req: &mut Request) -> IronResult<Response> {
    let mut file = if try!(session::is_logged_in(req)) {
        File::open("public/session_index.html").unwrap()
    } else {
        File::open("public/index.html").unwrap()
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Ok(Response::with((status::Ok, Header(ContentType::html()), contents)))
}

fn get_session(req: &mut Request) -> IronResult<Response> {
    let session = if try!(session::is_logged_in(req)) {
        session::get_username(req)
            .unwrap()
            .unwrap()
            .to_string()
    } else {
        String::from("No session")
    };

    Ok(Response::with((status::Ok, session)))
}

fn set_session(req: &mut Request) -> IronResult<Response> {
    try!(session::set_username(req, String::from("eh lemme")));
    Ok(Response::with((status::Ok, "set")))
}

fn delete_session(req: &mut Request) -> IronResult<Response> {
    try!(session::delete_username(req));
    Ok(Response::with((status::Ok, "Deleted")))
}

fn main() {
    let mut api_router = Router::new();
    api_router.get("/get_session", get_session, "get_session");
    api_router.get("/set_session", set_session, "set_session");
    api_router.get("/delete_session", delete_session, "delete_session");

    let session_secret = b"verysecret".to_vec();

    let mut mount = Mount::new();
    mount
        .mount("/", index)
        .mount("/api/", api_router)
        .mount("/public/", Static::new(Path::new("public/")));

    let mut ch = Chain::new(mount);
    ch.link_around(SessionStorage::new(SignedCookieBackend::new(session_secret)));
    let _res = Iron::new(ch).http("localhost:3000");
}
