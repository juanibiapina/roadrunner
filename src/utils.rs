extern crate hostname;

pub fn hostname() -> String {
    hostname::get_hostname().unwrap()
}
