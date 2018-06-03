use nom;
use types::*;

fn make_integration<'a>((name, exprs): (&'a str, Vec<Expr<'a>>)) -> Expr<'a> {
    Expr::Integration(Integration {
        name: name,
        exprs: exprs,
    })
}

pub fn parse(value: &str) -> Prompt {
    prompt(value).unwrap().1
}

named!(color<&str, Expr>,
    map!(
        alt!(
            delimited!(tag!("${"), nom::digit, char!('}')) => { |c: &str| Color::Ansi(c.parse::<u8>().unwrap()) } |
            delimited!(tag!("${"), nom::alpha, char!('}')) => { |c| Color::Name(color_name(c)) }
        ),
        |c| Expr::Color(c)
    )
);

named!(literal<&str, Expr>,
        map!(
            none_of!("{}%"),
            |c| Expr::Literal(Literal(c))
        )
);

named!(placeholder<&str, Expr>, map!(delimited!(char!('%'), nom::alpha, char!('%')), |c| Expr::Placeholder(Placeholder(c))));

named!(section<&str, Expr>, map!(delimited!(char!('{'), exprs, char!('}')), |c| Expr::Section(Section(c))));

named!(tagged_exprs<&str, (&str, Vec<Expr>)>, separated_pair!(nom::alpha, char!(':'), exprs));

named!(integration<&str, Expr>, map!(delimited!(char!('{'), tagged_exprs, char!('}')), make_integration));

named!(exprs<&str, Vec<Expr>>,
    many0!(
        alt_complete!(
            color       |
            literal     |
            placeholder |
            integration |
            section
        )
    )
);

named!(prompt<&str, Prompt>,
    map!(
        exprs,
        |exprs| Prompt { exprs: exprs }
    )
);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        assert_eq!(literal("S").unwrap(), ("", Expr::Literal(Literal('S'))));
        assert_eq!(literal("a").unwrap(), ("", Expr::Literal(Literal('a'))));
        assert_eq!(literal("[").unwrap(), ("", Expr::Literal(Literal('['))));
        assert_eq!(literal("]").unwrap(), ("", Expr::Literal(Literal(']'))));
        assert_eq!(literal(" ").unwrap(), ("", Expr::Literal(Literal(' '))));
        assert_eq!(literal("\n").unwrap(), ("", Expr::Literal(Literal('\n'))));
    }

    #[test]
    fn test_color() {
        assert_eq!(color("${red}").unwrap(), ("", Expr::Color(Color::Name(ColorName::Red))));
        assert_eq!(color("${blue}").unwrap(), ("", Expr::Color(Color::Name(ColorName::Blue))));
        assert_eq!(color("${reset}").unwrap(), ("", Expr::Color(Color::Name(ColorName::Reset))));
        assert_eq!(color("${1}").unwrap(), ("", Expr::Color(Color::Ansi(1))));
        assert_eq!(color("${100}").unwrap(), ("", Expr::Color(Color::Ansi(100))));
        //assert_eq!(color("${34,23,11}").unwrap(), ("", Expr::Color(ColorRgb(34,23,11))));
    }

    #[test]
    fn test_placeholder() {
        assert_eq!(placeholder("%hi%").unwrap(), ("", Expr::Placeholder(Placeholder("hi"))));
    }

    #[test]
    fn test_section() {
        assert_eq!(section("{[%hi%]}").unwrap(), ("", Expr::Section(Section(vec![
            Expr::Literal(Literal('[')),
            Expr::Placeholder(Placeholder("hi")),
            Expr::Literal(Literal(']')),
        ]))));
    }

    #[test]
    fn test_exprs() {
        assert_eq!(exprs("[%hi%]").unwrap(), ("", vec!(Expr::Literal(Literal('[')), Expr::Placeholder(Placeholder("hi")), Expr::Literal(Literal(']')))));
    }

    #[test]
    fn test_tagged_exprs() {
        assert_eq!(tagged_exprs("git:[%hi%]").unwrap(), ("", ("git", vec!(Expr::Literal(Literal('[')), Expr::Placeholder(Placeholder("hi")), Expr::Literal(Literal(']'))))));
    }

    #[test]
    fn test_integration() {
        assert_eq!(integration("{git:[%hi%]}").unwrap(), ("", Expr::Integration(Integration { name: "git", exprs: vec!(Expr::Literal(Literal('[')), Expr::Placeholder(Placeholder("hi")), Expr::Literal(Literal(']'))) })));
    }

    #[test]
    fn test_prompt() {
        assert_eq!(prompt("${11}{rbenv:${green}%version%}${22}[]{git:[%hi%{+-%status%}]}").unwrap(), ("", Prompt { exprs: vec![
            Expr::Color(Color::Ansi(11)),
            Expr::Integration(Integration {
                name: "rbenv",
                exprs: vec![
                    Expr::Color(Color::Name(ColorName::Green)),
                    Expr::Placeholder(Placeholder("version"))
                ]
            }),
            Expr::Color(Color::Ansi(22)),
            Expr::Literal(Literal('[')),
            Expr::Literal(Literal(']')),
            Expr::Integration(Integration {
                name: "git",
                exprs: vec![
                    Expr::Literal(Literal('[')),
                    Expr::Placeholder(Placeholder("hi")),
                    Expr::Section(Section(vec![
                        Expr::Literal(Literal('+')),
                        Expr::Literal(Literal('-')),
                        Expr::Placeholder(Placeholder("status")),
                    ])),
                    Expr::Literal(Literal(']'))
                ]
            })
        ]}));
    }
}
