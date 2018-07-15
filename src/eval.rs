extern crate termion;

use self::termion::color;

use types::*;
use utils;

pub fn eval(prompt: &Prompt) -> String {
    prompt.sections
        .iter()
        .filter_map(|section| eval_section(section))
        .map(|rendered_section| rendered_section.content)
        .collect::<Vec<String>>()
        .join("")
}

fn eval_section(section: &Section) -> Option<RenderedSection> {
    match section.name {
        Some(ref _name) => None,
        None => {
            Some(RenderedSection {
                content: section.parts
                            .iter()
                            .filter_map(|part| eval_part(part))
                            .collect::<Vec<_>>()
                            .join("")
            })
        },
    }
}

fn eval_part(part: &Part) -> Option<String> {
    match part {
        Part::Literal(value) => Some(value.to_owned()),
        Part::Interpolation(expr) => {
            let evaluated = eval_expr(expr);
            match evaluated {
                Expr::String(value) => Some(value),
                _ => None,
            }
        },
    }
}

fn eval_expr(expr: &Expr) -> Expr {
    match expr {
        Expr::FunctionCall(name, args) => {
            let evaluated_args = args.iter().map(eval_expr).collect::<Vec<_>>();
            invoke_function(name, evaluated_args.as_slice())
        },
        Expr::Variable(name) => Expr::String(name.to_owned()),
        Expr::String(value) => Expr::String(value.to_owned()),
    }
}

fn invoke_function(name: &str, args: &[Expr]) -> Expr {
    match name {
        "cwd" => Expr::String(utils::cwd()),
        "hostname" => Expr::String(utils::hostname()),
        "username" => Expr::String(utils::username()),
        "fg" => {
            match args[0] {
                Expr::String(ref value) => {
                    match value.as_ref() {
                        "reset" => Expr::String(format!("{}", color::Fg(color::Reset))),
                        "red" => Expr::String(format!("{}", color::Fg(color::Red))),
                        "black" => Expr::String(format!("{}", color::Fg(color::Black))),
                        "green" => Expr::String(format!("{}", color::Fg(color::Green))),
                        "yellow" => Expr::String(format!("{}", color::Fg(color::Yellow))),
                        "blue" => Expr::String(format!("{}", color::Fg(color::Blue))),
                        "magenta" => Expr::String(format!("{}", color::Fg(color::Magenta))),
                        "cyan" => Expr::String(format!("{}", color::Fg(color::Cyan))),
                        "white" => Expr::String(format!("{}", color::Fg(color::White))),
                        _ => Expr::String("".to_owned()),
                    }
                },
                _ => Expr::String("".to_owned()),
            }
        },
        "bg" => {
            match args[0] {
                Expr::String(ref value) => {
                    match value.as_ref() {
                        "reset" => Expr::String(format!("{}", color::Bg(color::Reset))),
                        "red" => Expr::String(format!("{}", color::Bg(color::Red))),
                        "black" => Expr::String(format!("{}", color::Bg(color::Black))),
                        "green" => Expr::String(format!("{}", color::Bg(color::Green))),
                        "yellow" => Expr::String(format!("{}", color::Bg(color::Yellow))),
                        "blue" => Expr::String(format!("{}", color::Bg(color::Blue))),
                        "magenta" => Expr::String(format!("{}", color::Bg(color::Magenta))),
                        "cyan" => Expr::String(format!("{}", color::Bg(color::Cyan))),
                        "white" => Expr::String(format!("{}", color::Bg(color::White))),
                        _ => Expr::String("".to_owned()),
                    }
                },
                _ => Expr::String("".to_owned()),
            }
        },
        _ => Expr::String("".to_owned()),
    }
}
