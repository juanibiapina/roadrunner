extern crate roadrunner;

use std::env;

fn main() {
    let prompt_string = match env::var("ROADRUNNER_PROMPT") {
        Ok(val) => val,
        Err(_) => return,
    };

    let engine = roadrunner::engine::Engine::new();

    let result = engine.run(&prompt_string);

    println!("{}", result);
}
