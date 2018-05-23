extern crate hostname;
extern crate users;

pub fn hostname() -> String {
    hostname::get_hostname().unwrap()
}

pub fn username() -> String {
    users::get_current_username().unwrap()
}
