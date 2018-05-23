#[derive(PartialEq, Debug)]
pub enum Expr<'a> {
    Literal(&'a str),
    Placeholder(&'a str),
}

#[derive(PartialEq, Debug)]
pub struct Section<'a> {
    pub name: &'a str,
    pub exprs: Vec<Expr<'a>>,
}

#[derive(PartialEq, Debug)]
pub enum TopLevelExpr<'a> {
    Expr(Expr<'a>),
    Section(Section<'a>),
}

#[derive(PartialEq, Debug)]
pub struct Prompt<'a> {
    pub exprs: Vec<TopLevelExpr<'a>>,
}
