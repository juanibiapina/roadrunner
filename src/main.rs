extern crate roadrunner;

extern crate clap;

use std::env;

use clap::{Arg, App};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("variable")
             .short("v")
             .long("variable")
             .value_name("VARIABLE")
             .help("Set the name of the environment variable to use")
             .takes_value(true))
        .get_matches();

    let variable = matches.value_of("variable").unwrap_or("ROADRUNNER_PROMPT");

    let prompt_string = match env::var(variable) {
        Ok(val) => val,
        Err(_) => return,
    };

    let engine = roadrunner::engine::Engine::new();

    let result = engine.run(&prompt_string);

    println!("{}", result);
}
