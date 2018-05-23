extern crate hostname;
extern crate users;

use std::env::current_dir;

pub fn hostname() -> String {
    hostname::get_hostname().unwrap()
}

pub fn username() -> String {
    users::get_current_username().unwrap()
}

pub fn cwd() -> String {
    current_dir().unwrap().to_str().unwrap().to_owned()
}
