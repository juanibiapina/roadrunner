extern crate roadrunner;
extern crate clap;

use clap::{Arg, App};

use std::process::exit;

use roadrunner::types::Error;

fn main() {
    let matches = App::new("roadrunner")
        .about("Shell prompt generator")
        .arg(Arg::with_name("SCRIPT")
             .help("Selects the script file to run")
             .required(true)
             .index(1))
        .get_matches();

    let script_filename = matches.value_of("SCRIPT").unwrap();

    let mut engine = roadrunner::engine::Engine::new();

    let result = engine.run_file(&script_filename);

    match result {
        Ok(result) => println!("{}", result),
        Err(Error::ScriptError(err)) => {
            println!("{}", err);
            exit(1);
        },
        Err(Error::IoError(err)) => {
            println!("{}", err);
            exit(1);
        },
    }
}
