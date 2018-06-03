use types::Context;
use utils;

pub struct TopLevelContext;

impl TopLevelContext {
    pub fn new() -> TopLevelContext {
        TopLevelContext
    }
}

impl Context for TopLevelContext {
    fn eval(&self, name: &str) -> String {
        match name {
            "cwd" => utils::cwd(),
            "hostname" => utils::hostname(),
            "username" => utils::username(),
            _ => "".to_owned(),
        }
    }
}
