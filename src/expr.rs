#[derive(PartialEq, Debug)]
pub enum Expr<'a> {
    Literal(&'a str),
    Placeholder(&'a str),
    Spec(Vec<Expr<'a>>),
    TaggedSpec(&'a str, Box<Expr<'a>>),
    Section(Box<Expr<'a>>),
    Prompt(Vec<Expr<'a>>),
}

