extern crate termion;

use self::termion::color;

use contexts::top_level::TopLevelContext;
use contexts::git::GitContext;
use contexts::rbenv::RbenvContext;
use types::*;

pub fn eval(prompt: &Prompt) -> String {
    let context = Box::new(TopLevelContext::new()) as Box<Context>;

    prompt.exprs.iter().map(|expr| eval_in_context(&context, expr)).collect::<Vec<String>>().join("")
}

fn eval_in_context(context: &Box<Context>, expr: &Expr) -> String {
    match expr {
        Expr::Color(color) => eval_color(color),
        Expr::Literal(literal) => literal.0.to_string(),
        Expr::Placeholder(placeholder) => context.eval(placeholder.0),
        Expr::Section(section) => {
            let context: Option<Box<Context>> = match section.name {
                "git" => GitContext::new().map(|i| Box::new(i) as Box<Context>),
                "rbenv" => RbenvContext::new().map(|i| Box::new(i) as Box<Context>),
                _ => panic!("unsupported section"),
            };

            let context = match context {
                Some(context) => context,
                None => return "".to_owned(),
            };

            section.exprs.iter().map(|expr| eval_in_context(&context, expr)).collect::<Vec<String>>().join("")
        },
    }
}

fn eval_color(color: &Color) -> String {
    match color {
        Color::Ansi(v) => format!("{}", color::Fg(color::AnsiValue(*v))),
        Color::Name(name) => {
            match name {
                ColorName::Reset => format!("{}", color::Fg(color::Reset)),
                ColorName::Black => format!("{}", color::Fg(color::Black)),
                ColorName::Red => format!("{}", color::Fg(color::Red)),
                ColorName::Green => format!("{}", color::Fg(color::Green)),
                ColorName::Yellow => format!("{}", color::Fg(color::Yellow)),
                ColorName::Blue => format!("{}", color::Fg(color::Blue)),
                ColorName::Magenta => format!("{}", color::Fg(color::Magenta)),
                ColorName::Cyan => format!("{}", color::Fg(color::Cyan)),
                ColorName::White => format!("{}", color::Fg(color::White)),
            }
        },
    }
}
