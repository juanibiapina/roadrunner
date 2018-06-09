#[derive(PartialEq, Debug)]
pub struct Literal(pub String);

#[derive(PartialEq, Debug)]
pub struct Placeholder<'a>(pub &'a str);

#[derive(PartialEq, Debug)]
pub struct Section<'a>(pub Vec<Expr<'a>>);

#[derive(PartialEq, Debug)]
pub struct Integration<'a> {
    pub name: &'a str,
    pub exprs: Vec<Expr<'a>>,
}

#[derive(PartialEq, Debug)]
pub enum Expr<'a> {
    Literal(Literal),
    Placeholder(Placeholder<'a>),
    Section(Section<'a>),
    Integration(Integration<'a>),
}

#[derive(PartialEq, Debug)]
pub struct Prompt<'a> {
    pub exprs: Vec<Expr<'a>>,
}

pub trait Context {
    fn eval(&self, expr: &str) -> EvalResult;
}

pub enum EvalResult {
    None,
    Some(String),
    Vec(Vec<EvalResult>),
}

impl EvalResult {
    pub fn simplify(self) -> Option<String> {
        match self {
            EvalResult::None => None,
            EvalResult::Some(value) => Some(value),
            EvalResult::Vec(values) => Some(values.into_iter().filter_map(|v| v.simplify()).collect::<Vec<String>>().join("")),
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            EvalResult::None => true,
            _ => false,
        }
    }
}

