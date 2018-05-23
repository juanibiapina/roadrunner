use nom;
use expr::*;

fn make_tagged_spec<'a>((name, spec): (&'a str, Expr<'a>)) -> Expr<'a> {
    Expr::TaggedSpec(name, Box::new(spec))
}

fn make_section<'a>(value: Expr<'a>) -> Expr<'a> {
    Expr::Section(Box::new(value))
}

pub fn parse(value: &str) -> Expr {
    prompt(value).unwrap().1
}

named!(literal<&str, Expr>, map!(alt!(tag!("[") | tag!("]")), Expr::Literal));

named!(placeholder<&str, Expr>, map!(delimited!(char!('%'), nom::alpha, char!('%')), Expr::Placeholder));

named!(spec<&str, Expr>, map!(many0!(alt_complete!(literal | placeholder)), Expr::Spec));

named!(tagged_spec<&str, Expr>, map!(separated_pair!(nom::alpha, char!(':'), spec), make_tagged_spec));

named!(section<&str, Expr>, map!(delimited!(char!('{'), tagged_spec, char!('}')), make_section));

named!(prompt<&str, Expr>, map!(many0!(alt_complete!(literal | section)), Expr::Prompt));


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
