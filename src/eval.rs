extern crate termion;

use self::termion::color;

use integrations;
use types::*;
use utils;

pub fn eval(prompt: &Prompt) -> String {
    prompt.exprs.iter().map(eval_top_level_expr).collect::<Vec<String>>().join("")
}

fn eval_top_level_expr(expr: &TopLevelExpr) -> String {
    match expr {
        TopLevelExpr::Expr(expr) => {
            match expr {
                Expr::Color(color) => eval_color(color),
                Expr::Literal(literal) => literal.0.to_string(),
                Expr::Placeholder(placeholder) => eval_top_level_placeholder(placeholder.0),
            }
        },
        TopLevelExpr::Section(value) => {
            eval_section(value)
        }
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

fn eval_section(section: &Section) -> String {
    let integration: Option<Box<Integration>> = match section.name {
        "git" => integrations::git::Git::new().map(|i| Box::new(i) as Box<Integration>),
        "rbenv" => integrations::rbenv::Rbenv::new().map(|i| Box::new(i) as Box<Integration>),
        _ => panic!("unsupported section"),
    };

    let integration = match integration {
        Some(integration) => integration,
        None => return "".to_owned(),
    };

    section.exprs.iter().map(|expr| eval_in_integration(expr, &integration)).collect::<Vec<String>>().join("")
}

fn eval_top_level_placeholder(name: &str) -> String {
    match name {
        "hostname" => utils::hostname(),
        "username" => utils::username(),
        "cwd" => utils::cwd(),
        _ => panic!("unsupported placeholder"),
    }
}

fn eval_in_integration(expr: &Expr, integration: &Box<Integration>) -> String {
    match expr {
        Expr::Color(color) => eval_color(color),
        Expr::Literal(literal) => literal.0.to_string(),
        Expr::Placeholder(placeholder) => {
            integration.eval(placeholder)
        },
    }
}
