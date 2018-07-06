#[derive(PartialEq, Debug)]
pub struct Section {
    pub name: Option<String>,
    pub parts: Vec<Part>,
}

#[derive(PartialEq, Debug)]
pub enum Part {
    Literal(String),
    Interpolation(Expr),
}

#[derive(PartialEq, Debug)]
pub enum Expr {
    FunctionCall(String, Vec<Expr>),
    Variable(String),
    String(String),
}

#[derive(PartialEq, Debug)]
pub struct Prompt {
    pub sections: Vec<Section>,
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

