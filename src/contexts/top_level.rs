use types::Context;
use types::EvalResult;
use utils;

pub struct TopLevelContext;

impl TopLevelContext {
    pub fn new() -> TopLevelContext {
        TopLevelContext
    }
}

impl Context for TopLevelContext {
    fn eval(&self, name: &str) -> EvalResult {
        match name {
            "cwd" => EvalResult::Some(utils::cwd()),
            "hostname" => EvalResult::Some(utils::hostname()),
            "username" => EvalResult::Some(utils::username()),
            _ => EvalResult::None,
        }
    }
}
