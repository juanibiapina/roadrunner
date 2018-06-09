use nom;
use nom::types::CompleteStr;

use types::*;

fn make_integration<'a>((name, exprs): (&'a str, Vec<Expr<'a>>)) -> Expr<'a> {
    Expr::Integration(Integration {
        name: name,
        exprs: exprs,
    })
}

pub fn parse(value: &str) -> Prompt {
    prompt(CompleteStr(value)).unwrap().1
}

named!(literal<CompleteStr, Expr>,
        map!(
            none_of!("{}%"),
            |c| Expr::Literal(Literal(c))
        )
);

named!(placeholder<CompleteStr, Expr>, map!(delimited!(char!('%'), nom::alphanumeric, char!('%')), |c| Expr::Placeholder(Placeholder(c.0))));

named!(section<CompleteStr, Expr>, map!(delimited!(char!('{'), exprs, char!('}')), |c| Expr::Section(Section(c))));

named!(tagged_exprs<CompleteStr, (&str, Vec<Expr>)>, separated_pair!(map!(nom::alpha, |e| e.0), char!(':'), exprs));

named!(integration<CompleteStr, Expr>, map!(delimited!(tag!("#{"), tagged_exprs, char!('}')), make_integration));

named!(exprs<CompleteStr, Vec<Expr>>,
    many0!(
        alt_complete!(
            section     |
            placeholder |
            integration |
            literal
        )
    )
);

named!(prompt<CompleteStr, Prompt>,
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
        assert_eq!(literal(CompleteStr("S")).unwrap(), (CompleteStr(""), Expr::Literal(Literal('S'))));
        assert_eq!(literal(CompleteStr("a")).unwrap(), (CompleteStr(""), Expr::Literal(Literal('a'))));
        assert_eq!(literal(CompleteStr("[")).unwrap(), (CompleteStr(""), Expr::Literal(Literal('['))));
        assert_eq!(literal(CompleteStr("]")).unwrap(), (CompleteStr(""), Expr::Literal(Literal(']'))));
        assert_eq!(literal(CompleteStr(" ")).unwrap(), (CompleteStr(""), Expr::Literal(Literal(' '))));
        assert_eq!(literal(CompleteStr("\n")).unwrap(), (CompleteStr(""), Expr::Literal(Literal('\n'))));
    }

    #[test]
    fn test_placeholder() {
        assert_eq!(placeholder(CompleteStr("%hi%")).unwrap(), (CompleteStr(""), Expr::Placeholder(Placeholder("hi"))));
        assert_eq!(placeholder(CompleteStr("%1234%")).unwrap(), (CompleteStr(""), Expr::Placeholder(Placeholder("1234"))));
    }

    #[test]
    fn test_section() {
        assert_eq!(section(CompleteStr("{[%hi%]}")).unwrap(), (CompleteStr(""), Expr::Section(Section(vec![
            Expr::Literal(Literal('[')),
            Expr::Placeholder(Placeholder("hi")),
            Expr::Literal(Literal(']')),
        ]))));
    }

    #[test]
    fn test_exprs() {
        assert_eq!(exprs(CompleteStr("[%hi%]")).unwrap(), (CompleteStr(""), vec!(Expr::Literal(Literal('[')), Expr::Placeholder(Placeholder("hi")), Expr::Literal(Literal(']')))));
    }

    #[test]
    fn test_tagged_exprs() {
        assert_eq!(tagged_exprs(CompleteStr("git:[%hi%]")).unwrap(), (CompleteStr(""), ("git", vec!(Expr::Literal(Literal('[')), Expr::Placeholder(Placeholder("hi")), Expr::Literal(Literal(']'))))));
    }

    #[test]
    fn test_integration() {
        assert_eq!(integration(CompleteStr("#{git:[%hi%]}")).unwrap(), (CompleteStr(""), Expr::Integration(Integration { name: "git", exprs: vec!(Expr::Literal(Literal('[')), Expr::Placeholder(Placeholder("hi")), Expr::Literal(Literal(']'))) })));
    }

    #[test]
    fn test_prompt() {
        assert_eq!(prompt(CompleteStr("#{fg:%11%}#{rbenv:#{bg:%green%}%version%}#{fg:%22%}[]#{git:[%hi%{+-%status%}]}")).unwrap(), (CompleteStr(""), Prompt { exprs: vec![
            Expr::Integration(Integration {
                name: "fg",
                exprs: vec![
                    Expr::Placeholder(Placeholder("11")),
                ]
            }),
            Expr::Integration(Integration {
                name: "rbenv",
                exprs: vec![
                    Expr::Integration(Integration {
                        name: "bg",
                        exprs: vec![
                            Expr::Placeholder(Placeholder("green")),
                        ]
                    }),
                    Expr::Placeholder(Placeholder("version"))
                ]
            }),
            Expr::Integration(Integration {
                name: "fg",
                exprs: vec![
                    Expr::Placeholder(Placeholder("22")),
                ]
            }),
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
