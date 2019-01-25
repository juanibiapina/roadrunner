use std::collections::HashMap;

use types::*;

pub struct Context {
    entries: HashMap<String, Expr>,
}

impl Context {
    pub fn top_level() -> Context {
        let entries = hashmap!{
            "reset".to_owned() => Expr::String("reset".to_owned()),
            "black".to_owned() => Expr::String("black".to_owned()),
            "red".to_owned() => Expr::String("red".to_owned()),
            "green".to_owned() => Expr::String("green".to_owned()),
            "yellow".to_owned() => Expr::String("yellow".to_owned()),
            "blue".to_owned() => Expr::String("blue".to_owned()),
            "magenta".to_owned() => Expr::String("magenta".to_owned()),
            "cyan".to_owned() => Expr::String("cyan".to_owned()),
            "white".to_owned() => Expr::String("white".to_owned()),
        };

        Context {
            entries,
        }
    }

    pub fn get(&self, name: &str) -> Expr {
        self.entries.get(name).unwrap().clone()
    }
}

