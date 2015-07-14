#![crate_name = "stackoverflow"]
#![crate_type = "rlib"]
extern crate curl;
extern crate rustc_serialize;

use client::Client;
use user::UserService;

pub struct Stackoverflow {
    pub user: UserService,
}

impl Stackoverflow {
    pub fn new() -> Stackoverflow {
        let client = Client;
        Stackoverflow {
            user: UserService::new(client),
        }
    }
}

pub mod client;
pub mod user;
