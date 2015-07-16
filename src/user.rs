#[allow(unused_attributes)]

use rustc_serialize::json;
use client::Client;

pub struct UserService {
    client: Client,
}

impl UserService {

    pub fn new(c: Client) -> UserService {
        UserService { client: c }
    }

    pub fn get(self) -> User {
        let url = "http://api.stackexchange.com/2.2/users/2357380?site=stackoverflow".to_string();

        let req = self.client.request(url.as_ref());

        let user: User = json::decode(req.as_ref()).unwrap();

        return user;
    }
}

#[allow(dead_code)]
#[derive(RustcDecodable)]
pub struct User {
    pub items: String,
    pub has_more: String,
}
