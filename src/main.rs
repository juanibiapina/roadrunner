#[macro_use]
extern crate nom;

use std::env;

#[derive(PartialEq, Debug)]
enum Expr<'a> {
    Literal(&'a str),
    Placeholder(&'a str),
    Spec(Vec<Expr<'a>>),
    TaggedSpec(&'a str, Box<Expr<'a>>),
    Section(Box<Expr<'a>>),
    Prompt(Vec<Expr<'a>>),
}

fn make_tagged_spec<'a>((name, spec): (&'a str, Expr<'a>)) -> Expr<'a> {
    Expr::TaggedSpec(name, Box::new(spec))
}

fn make_section<'a>(value: Expr<'a>) -> Expr<'a> {
    Expr::Section(Box::new(value))
}

named!(literal<&str, Expr>, map!(alt!(tag!("[") | tag!("]")), Expr::Literal));

named!(placeholder<&str, Expr>, map!(delimited!(char!('%'), nom::alpha, char!('%')), Expr::Placeholder));

named!(spec<&str, Expr>, map!(many0!(alt_complete!(literal | placeholder)), Expr::Spec));

named!(tagged_spec<&str, Expr>, map!(separated_pair!(nom::alpha, char!(':'), spec), make_tagged_spec));

named!(section<&str, Expr>, map!(delimited!(char!('{'), tagged_spec, char!('}')), make_section));

named!(prompt<&str, Expr>, map!(many0!(alt_complete!(literal | section)), Expr::Prompt));

fn main() {
    let prompt_string = match env::var("ROADRUNNER_PROMPT") {
        Ok(val) => val,
        Err(_) => return,
    };

    let parsed = prompt(&prompt_string).unwrap().1;
    let result = eval(&parsed);

    println!("{}", result);
}

fn eval(expr: &Expr) -> String {
    match expr {
        Expr::Prompt(expr) => expr.iter().map(eval).collect::<Vec<String>>().join(""),
        Expr::Literal(value) => value.to_string(),
        Expr::Section(expr) => eval(expr),
        Expr::TaggedSpec(tag, expr) => eval_spec(tag, expr),
        _ => panic!("invariant violated"),
    }
}

fn eval_spec(tag: &str, expr: &Expr) -> String {
    match expr {
        Expr::Spec(exprs) => {
            match tag {
                "git" => "git".to_owned(),
                "rbenv" => "rbenv".to_owned(),
                _ => "unsupported_tag".to_owned(),
            }
        },
        _ => panic!("invariant violated"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        assert_eq!(literal("[").unwrap(), ("", Expr::Literal("[")));
        assert_eq!(literal("]").unwrap(), ("", Expr::Literal("]")));
    }

    #[test]
    fn test_placeholder() {
        assert_eq!(placeholder("%hi%").unwrap(), ("", Expr::Placeholder("hi")));
    }

    #[test]
    fn test_spec() {
        assert_eq!(spec("[%hi%]").unwrap(), ("", Expr::Spec(vec!(Expr::Literal("["), Expr::Placeholder("hi"), Expr::Literal("]")))));
    }

    #[test]
    fn test_tagged_spec() {
        assert_eq!(tagged_spec("git:[%hi%]").unwrap(), ("", Expr::TaggedSpec("git", Box::new(Expr::Spec(vec!(Expr::Literal("["), Expr::Placeholder("hi"), Expr::Literal("]")))))));
    }

    #[test]
    fn test_section() {
        assert_eq!(section("{git:[%hi%]}").unwrap(), ("", Expr::Section(Box::new(Expr::TaggedSpec("git", Box::new(Expr::Spec(vec!(Expr::Literal("["), Expr::Placeholder("hi"), Expr::Literal("]")))))))));
    }

    #[test]
    fn test_prompt() {
        assert_eq!(prompt("{rbenv:%version%}[]{git:[%hi%]}").unwrap(), ("", Expr::Prompt(vec![Expr::Section(Box::new(Expr::TaggedSpec("rbenv", Box::new(Expr::Spec(vec!(Expr::Placeholder("version"))))))), Expr::Literal("["), Expr::Literal("]"), Expr::Section(Box::new(Expr::TaggedSpec("git", Box::new(Expr::Spec(vec!(Expr::Literal("["), Expr::Placeholder("hi"), Expr::Literal("]")))))))])));
    }
}
