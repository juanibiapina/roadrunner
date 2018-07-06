use nom;
use nom::types::CompleteStr;

use types::*;

pub fn parse(value: &str) -> Prompt {
    prompt(CompleteStr(value)).unwrap().1
}

named!(literal<CompleteStr, Part>,
    map!(
        many_till!(
            take!(1),
            peek!(
                alt!(
                    tag!(";")  |
                    tag!("#{") |
                    eof!()
                )
            )
        ),
        |(strs, _)| Part::Literal(strs.iter().map(|s| s.0.to_owned()).collect::<Vec<_>>().join(""))
    )
);

named!(expr<CompleteStr, Expr>,
    alt!(
        function_call |
        variable
    )
);

named!(args<CompleteStr, Vec<Expr>>,
    delimited!(
        char!('('),
        many0!(expr),
        char!(')')
    )
);

named!(function_call<CompleteStr, Expr>,
    map!(
        pair!(nom::alphanumeric, args),
        |(name, args)| Expr::FunctionCall(name.0.to_owned(), args)
    )
);

named!(variable<CompleteStr, Expr>,
    map!(
        nom::alphanumeric,
        |s| Expr::Variable(s.0.to_owned())
    )
);

named!(interpolation<CompleteStr, Part>,
    map!(
        delimited!(tag!("#{"), expr, char!('}')),
        |expr| Part::Interpolation(expr)
    )
);

named!(section<CompleteStr, Section>,
    map!(
        pair!(
            opt!(
                map!(
                    delimited!(char!('?'), nom::alphanumeric, char!(':')),
                    |s| s.0.to_owned()
                )
            ),
            many1!(
                alt!(
                    interpolation |
                    literal
                )
            )
        ),
        |(name, parts)| Section { name, parts }
    )
);

named!(prompt<CompleteStr, Prompt>,
    alt!(
        map!(
            separated_list!(char!(';'), section),
            |sections| Prompt { sections: sections }
        ) |
        map!(
            eof!(),
            |_| Prompt { sections: Vec::new() }
        )
    )
);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        assert_eq!(literal(CompleteStr("String")).unwrap(), (CompleteStr(""), Part::Literal("String".to_owned())));
        assert_eq!(literal(CompleteStr("[]@{}  ()=")).unwrap(), (CompleteStr(""), Part::Literal("[]@{}  ()=".to_owned())));
        assert_eq!(literal(CompleteStr("a\nb")).unwrap(), (CompleteStr(""), Part::Literal("a\nb".to_owned())));
        assert_eq!(literal(CompleteStr("\u{1b}[39m")).unwrap(), (CompleteStr(""), Part::Literal("\u{1b}[39m".to_owned())));
    }

    #[test]
    fn test_section_without_name() {
        assert_eq!(section(CompleteStr("part1#{name}part2")).unwrap(), (CompleteStr(""), Section {
            name: None,
            parts: vec![
                Part::Literal("part1".to_owned()),
                Part::Interpolation(Expr::Variable("name".to_owned())),
                Part::Literal("part2".to_owned()),
            ]
        }));
    }

    #[test]
    fn test_section_with_name() {
        assert_eq!(section(CompleteStr("?name:part1#{name}part2")).unwrap(), (CompleteStr(""), Section {
            name: Some("name".to_owned()),
            parts: vec![
                Part::Literal("part1".to_owned()),
                Part::Interpolation(Expr::Variable("name".to_owned())),
                Part::Literal("part2".to_owned()),
            ]
        }));
    }

    #[test]
    fn test_prompt() {
        assert_eq!(prompt(CompleteStr("")).unwrap(), (CompleteStr(""), Prompt { sections: Vec::new() }));
    }
}
