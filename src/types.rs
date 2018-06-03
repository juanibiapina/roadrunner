#[derive(PartialEq, Debug)]
pub struct Literal(pub char);

#[derive(PartialEq, Debug)]
pub struct Placeholder<'a>(pub &'a str);

#[derive(PartialEq, Debug)]
pub enum ColorName {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(PartialEq, Debug)]
pub enum Color {
    Ansi(u8),
    Name(ColorName),
}

#[derive(PartialEq, Debug)]
pub enum Expr<'a> {
    Color(Color),
    Literal(Literal),
    Placeholder(Placeholder<'a>),
    Section(Section<'a>),
}

#[derive(PartialEq, Debug)]
pub struct Section<'a> {
    pub name: &'a str,
    pub exprs: Vec<Expr<'a>>,
}

#[derive(PartialEq, Debug)]
pub struct Prompt<'a> {
    pub exprs: Vec<Expr<'a>>,
}

pub trait Context {
    fn eval(&self, expr: &str) -> String;
}

pub fn color_name(n: &str) -> ColorName {
    match n {
        "reset" => ColorName::Reset,
        "black" => ColorName::Black,
        "red" => ColorName::Red,
        "green" => ColorName::Green,
        "yellow" => ColorName::Yellow,
        "blue" => ColorName::Blue,
        "magenta" => ColorName::Magenta,
        "cyan" => ColorName::Cyan,
        "white" => ColorName::White,
        _ => panic!("unsupported color"),
    }
}
