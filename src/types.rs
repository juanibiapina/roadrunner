#[derive(PartialEq, Debug)]
pub struct Section {
    pub name: Option<String>,
    pub parts: Vec<Part>,
}

#[derive(PartialEq, Debug)]
pub enum Part {
    Literal(String),
    Interpolation(Expr),
    Conditional(Vec<Part>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    FunctionCall(String, Vec<Expr>),
    Variable(String),
    Trigger(String),
    String(String),
}

#[derive(PartialEq, Debug)]
pub struct Prompt {
    pub sections: Vec<Section>,
}

#[derive(PartialEq, Debug)]
pub struct RenderedSection {
    pub content: String,
}

#[derive(PartialEq, Debug)]
pub struct RenderedPart {
    pub content: String,
    pub trigger: bool,
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
}

