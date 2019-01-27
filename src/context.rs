use std::collections::HashMap;

use types::*;

pub struct Context<'a> {
    entries: HashMap<String, Expr>,
    parent: Option<&'a Context<'a>>,
}

impl<'a> Context<'a> {
    pub fn new(parent: Option<&'a Context>) -> Context<'a> {
        Context {
            entries: HashMap::new(),
            parent: parent,
        }
    }

    pub fn top_level() -> Context<'a> {
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
            parent: None,
        }
    }

    pub fn set(&mut self, name: &str, value: &Expr) {
        self.entries.insert(name.to_owned(), value.clone());
    }

    pub fn get(&self, name: &str) -> Expr {
        match self.entries.get(name) {
            Some(value) => value.clone(),
            None => {
                match self.parent {
                    Some(parent) => parent.get(name),
                    None => panic!("Undefined variable: {}", name),
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_no_parent() {
        let mut context = Context::new(None);
        let expr = Expr::String("whatever".to_owned());

        context.set("name", &expr);

        assert_eq!(context.get("name"), expr);
    }

    #[test]
    fn test_context_with_one_parent() {
        let parent = Context::top_level();
        let context = Context::new(Some(&parent));

        assert_eq!(context.get("red"), Expr::String("red".to_owned()));
    }

    #[test]
    fn test_context_with_more_parents() {
        let parent = Context::top_level();
        let mid_context = Context::new(Some(&parent));
        let context = Context::new(Some(&mid_context));

        assert_eq!(context.get("red"), Expr::String("red".to_owned()));
    }
}
