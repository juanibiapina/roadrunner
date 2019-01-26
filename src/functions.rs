extern crate termion;
extern crate hostname;
extern crate users;

use self::termion::color;

use std::env;
use types::*;

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
                    #[allow(deprecated)]
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

pub fn fg(args: &[Expr]) -> Expr {
    match args[0] {
        Expr::String(ref value) => {
            match value.as_ref() {
                "reset" => Expr::String(format!("{}", color::Fg(color::Reset))),
                "red" => Expr::String(format!("{}", color::Fg(color::Red))),
                "black" => Expr::String(format!("{}", color::Fg(color::Black))),
                "green" => Expr::String(format!("{}", color::Fg(color::Green))),
                "yellow" => Expr::String(format!("{}", color::Fg(color::Yellow))),
                "blue" => Expr::String(format!("{}", color::Fg(color::Blue))),
                "magenta" => Expr::String(format!("{}", color::Fg(color::Magenta))),
                "cyan" => Expr::String(format!("{}", color::Fg(color::Cyan))),
                "white" => Expr::String(format!("{}", color::Fg(color::White))),
                _ => panic!("Unknown color: {}", value),
            }
        },
        _ => Expr::String("".to_owned()),
    }
}

pub fn bg(args: &[Expr]) -> Expr {
    match args[0] {
        Expr::String(ref value) => {
            match value.as_ref() {
                "reset" => Expr::String(format!("{}", color::Bg(color::Reset))),
                "red" => Expr::String(format!("{}", color::Bg(color::Red))),
                "black" => Expr::String(format!("{}", color::Bg(color::Black))),
                "green" => Expr::String(format!("{}", color::Bg(color::Green))),
                "yellow" => Expr::String(format!("{}", color::Bg(color::Yellow))),
                "blue" => Expr::String(format!("{}", color::Bg(color::Blue))),
                "magenta" => Expr::String(format!("{}", color::Bg(color::Magenta))),
                "cyan" => Expr::String(format!("{}", color::Bg(color::Cyan))),
                "white" => Expr::String(format!("{}", color::Bg(color::White))),
                _ => panic!("Unknown color: {}", value),
            }
        },
        _ => Expr::String("".to_owned()),
    }
}

pub fn tr(args: &[Expr]) -> Expr {
    match args[0] {
        Expr::String(ref value) => Expr::Trigger(value.to_owned()),
        _ => Expr::Trigger("".to_owned()),
    }
}
