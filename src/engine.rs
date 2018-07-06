use contexts::top_level::TopLevelContext;
use parser;
use eval;

pub struct Engine {
    top_level_context: TopLevelContext,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            top_level_context: TopLevelContext::new(),
        }
    }

    pub fn run(&self, input: &str) -> String {
        let parsed = parser::parse(input);
        eval::eval(&parsed)
    }
}
