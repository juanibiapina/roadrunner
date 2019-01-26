use parser;
use eval;

pub struct Engine;

impl Engine {
    pub fn new() -> Engine {
        Engine
    }

    pub fn run(&self, input: &str) -> String {
        let parsed = parser::parse(input);
        eval::eval(&parsed)
    }
}
