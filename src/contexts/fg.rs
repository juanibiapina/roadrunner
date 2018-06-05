extern crate termion;

use self::termion::color;
use types::*;

pub struct FgContext;

impl FgContext {
    pub fn new() -> Option<FgContext> {
        Some(FgContext)
    }
}

impl Context for FgContext {
    fn eval(&self, name: &str) -> EvalResult {
        match name {
            "reset" => EvalResult::Some(format!("{}", color::Fg(color::Reset))),
            "red" => EvalResult::Some(format!("{}", color::Fg(color::Red))),
            "black" => EvalResult::Some(format!("{}", color::Fg(color::Black))),
            "green" => EvalResult::Some(format!("{}", color::Fg(color::Green))),
            "yellow" => EvalResult::Some(format!("{}", color::Fg(color::Yellow))),
            "blue" => EvalResult::Some(format!("{}", color::Fg(color::Blue))),
            "magenta" => EvalResult::Some(format!("{}", color::Fg(color::Magenta))),
            "cyan" => EvalResult::Some(format!("{}", color::Fg(color::Cyan))),
            "white" => EvalResult::Some(format!("{}", color::Fg(color::White))),
            _ => EvalResult::None,
        }
    }
}
