#[macro_use]
extern crate nom;

mod types;
mod parser;
mod eval;

pub fn process(value: &str) -> String {
    let parsed = parser::parse(value);
    eval::eval(&parsed)
}
