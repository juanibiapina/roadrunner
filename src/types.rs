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
    Number(u8),
    Boolean(bool),
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
