#[macro_use] extern crate iron;
extern crate iron_sessionstorage;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate urlencoded;
#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate diesel_codegen;
extern crate rustc_serialize;
extern crate regex;
extern crate bcrypt;

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
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

pub mod schema;
pub mod models;
pub mod users;
pub mod utils;

use users::session;

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

fn main() {
    let mut api_router = Router::new();
    api_router.post("/register", users::api::register, "register");
    api_router.post("/login", users::api::login, "login");
    api_router.post("/logout", users::api::logout, "logout");
    api_router.get("/get_user", users::api::get_user, "get_user");
    api_router.post("/update_user", users::api::update_user, "update_user");
    api_router.post("/delete_user", users::api::delete_user, "delete_user");

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
