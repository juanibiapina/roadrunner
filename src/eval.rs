extern crate termion;

use self::termion::color;

use contexts::top_level::TopLevelContext;
use contexts::git::GitContext;
use contexts::rbenv::RbenvContext;
use types::*;

pub fn eval(prompt: &Prompt) -> String {
    let context = Box::new(TopLevelContext::new()) as Box<Context>;

    prompt.exprs
        .iter()
        .map(|expr| eval_in_context(&context, expr))
        .filter_map(|result| result.simplify())
        .collect::<Vec<String>>()
        .join("")
}

fn eval_in_context(context: &Box<Context>, expr: &Expr) -> EvalResult {
    match expr {
        Expr::Color(color) => eval_color(color),
        Expr::Literal(literal) => EvalResult::Some(literal.0.to_string()),
        Expr::Placeholder(placeholder) => context.eval(placeholder.0),
        Expr::Section(section) => eval_section(context, section),
        Expr::Integration(integration) => {
            let context: Option<Box<Context>> = match integration.name {
                "git" => GitContext::new().map(|i| Box::new(i) as Box<Context>),
                "rbenv" => RbenvContext::new().map(|i| Box::new(i) as Box<Context>),
                _ => panic!("unsupported integration"),
            };

            let context = match context {
                Some(context) => context,
                None => return EvalResult::None,
            };

            EvalResult::Vec(integration.exprs
                .iter()
                .map(|expr| eval_in_context(&context, expr))
                .collect::<Vec<EvalResult>>()
            )
        },
    }
}

fn eval_color(color: &Color) -> EvalResult {
    EvalResult::Some(match color.typ {
        ColorType::Fg => {
            match color.value {
                ColorValue::Ansi(v) => format!("{}", color::Fg(color::AnsiValue(v))),
                ColorValue::Name(ref name) => {
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
        },
        ColorType::Bg => {
            match color.value {
                ColorValue::Ansi(v) => format!("{}", color::Bg(color::AnsiValue(v))),
                ColorValue::Name(ref name) => {
                    match name {
                        ColorName::Reset => format!("{}", color::Bg(color::Reset)),
                        ColorName::Black => format!("{}", color::Bg(color::Black)),
                        ColorName::Red => format!("{}", color::Bg(color::Red)),
                        ColorName::Green => format!("{}", color::Bg(color::Green)),
                        ColorName::Yellow => format!("{}", color::Bg(color::Yellow)),
                        ColorName::Blue => format!("{}", color::Bg(color::Blue)),
                        ColorName::Magenta => format!("{}", color::Bg(color::Magenta)),
                        ColorName::Cyan => format!("{}", color::Bg(color::Cyan)),
                        ColorName::White => format!("{}", color::Bg(color::White)),
                    }
                },
            }
        },
    })
}

fn eval_section(context: &Box<Context>, section: &Section) -> EvalResult {
    let mut render = false;

    let results = section.0.iter().map(|expr| {
        match expr {
            Expr::Placeholder(_) | Expr::Section(_) => {
                let result = eval_in_context(context, expr);

                if !result.is_none() {
                    render = true;
                }

                result
            },
            _ => eval_in_context(context, expr),
        }
    }).collect::<Vec<EvalResult>>();

    if render {
        EvalResult::Vec(results)
    } else {
        EvalResult::None
    }
}
