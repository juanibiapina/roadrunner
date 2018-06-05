extern crate termion;

use self::termion::color;
use types::*;

pub struct BgContext;

impl BgContext {
    pub fn new() -> Option<BgContext> {
        Some(BgContext)
    }
}

impl Context for BgContext {
    fn eval(&self, name: &str) -> EvalResult {
        match name {
            "reset" => EvalResult::Some(format!("{}", color::Bg(color::Reset))),
            "red" => EvalResult::Some(format!("{}", color::Bg(color::Red))),
            "black" => EvalResult::Some(format!("{}", color::Bg(color::Black))),
            "green" => EvalResult::Some(format!("{}", color::Bg(color::Green))),
            "yellow" => EvalResult::Some(format!("{}", color::Bg(color::Yellow))),
            "blue" => EvalResult::Some(format!("{}", color::Bg(color::Blue))),
            "magenta" => EvalResult::Some(format!("{}", color::Bg(color::Magenta))),
            "cyan" => EvalResult::Some(format!("{}", color::Bg(color::Cyan))),
            "white" => EvalResult::Some(format!("{}", color::Bg(color::White))),
            _ => EvalResult::None,
        }
    }
}
