#[derive(PartialEq, Debug)]
pub struct Literal<'a>(pub &'a str);

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
