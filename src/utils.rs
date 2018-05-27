extern crate hostname;
extern crate users;

use std::env;

pub fn hostname() -> String {
    hostname::get_hostname().unwrap()
}

pub fn username() -> String {
    users::get_current_username().unwrap()
}

pub fn cwd() -> String {
    match env::current_dir() {
        Ok(current) => {
            match current.to_str() {
                Some(current) => {
                    match env::home_dir() {
                        Some(home) => {
                            match home.to_str() {
                                Some(home) => {
                                    current.replacen(home, "~", 1)
                                },
                                None => current.to_owned(),
                            }
                        },
                        None => current.to_owned(),
                    }
                },
                None => "".to_owned(),
            }
        },
        Err(_) => "".to_owned(),
    }
}
