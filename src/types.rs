#[derive(PartialEq, Debug)]
pub struct Literal<'a>(pub &'a str);

#[derive(PartialEq, Debug)]
pub struct Placeholder<'a>(pub &'a str);

#[derive(PartialEq, Debug)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    Placeholder(Placeholder<'a>),
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

pub trait Integration {
    fn eval(&self, expr: &Placeholder) -> String;
}
