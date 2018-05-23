extern crate roadrunner;

use std::env;

fn main() {
    let prompt_string = match env::var("ROADRUNNER_PROMPT") {
        Ok(val) => val,
        Err(_) => return,
    };

    let result = roadrunner::process(&prompt_string);

    println!("{}", result);
}
