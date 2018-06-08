#![feature(path_ancestors)]

#[macro_use]
extern crate nom;

mod types;
mod parser;
mod eval;
mod utils;
mod contexts;
pub mod engine;
