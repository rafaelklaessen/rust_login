use iron::prelude::*;
use iron_sessionstorage;
use iron_sessionstorage::traits::*;

pub struct Username(pub String);

impl Username {
    pub fn to_string(self) -> String {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.trim().is_empty()
    }
}

impl iron_sessionstorage::Value for Username {
    fn get_key() -> &'static str { "username" }
    fn into_raw(self) -> String { self.0 }
    fn from_raw(value: String) -> Option<Self> {
        Some(Username(value))
    }
}

pub fn is_logged_in(req: &mut Request) -> Result<bool, IronError> {
    match get_username(req)? {
        Some(username) => Ok(!username.is_empty()),
        None => Ok(false)
    }
}

pub fn get_username(req: &mut Request) -> Result<Option<Username>, IronError> {
    req.session().get::<Username>()
}

pub fn set_username(req: &mut Request, username: String) -> Result<(), IronError> {
    req.session().set(Username(username))
}

pub fn delete_username(req: &mut Request) -> Result<(), IronError> {
    set_username(req, "".to_owned())
}
