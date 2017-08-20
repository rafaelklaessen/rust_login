extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;

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

fn index(_req: &mut Request) -> IronResult<Response> {
    let mut file = File::open("public/index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Ok(Response::with((status::Ok, Header(ContentType::html()), contents)))
}

fn main() {
    let mut router = Router::new();
    router.get("/", index, "index");

    let mut mount = Mount::new();
    mount
        .mount("/", router)
        .mount("/public/", Static::new(Path::new("public/")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
