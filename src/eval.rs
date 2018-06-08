extern crate termion;

use std::borrow::Borrow;

use contexts::git::GitContext;
use contexts::fg::FgContext;
use contexts::bg::BgContext;
use contexts::rbenv::RbenvContext;
use types::*;

pub fn eval<T: Borrow<Context>>(context: T, prompt: &Prompt) -> String {
    prompt.exprs
        .iter()
        .map(|expr| eval_in_context(context.borrow(), expr))
        .filter_map(|result| result.simplify())
        .collect::<Vec<String>>()
        .join("")
}

fn eval_in_context<T: Borrow<Context>>(context: T, expr: &Expr) -> EvalResult {
    match expr {
        Expr::Literal(literal) => EvalResult::Some(literal.0.to_string()),
        Expr::Placeholder(placeholder) => context.borrow().eval(placeholder.0),
        Expr::Section(section) => eval_section(context, section),
        Expr::Integration(integration) => {
            let context: Option<Box<Context>> = match integration.name {
                "fg" => FgContext::new().map(|i| Box::new(i) as Box<Context>),
                "bg" => BgContext::new().map(|i| Box::new(i) as Box<Context>),
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
                .map(|expr| eval_in_context(context.borrow(), expr))
                .collect::<Vec<EvalResult>>()
            )
        },
    }
}

fn eval_section<T: Borrow<Context>>(context: T, section: &Section) -> EvalResult {
    let mut render = false;

    let results = section.0.iter().map(|expr| {
        match expr {
            Expr::Placeholder(_) | Expr::Section(_) => {
                let result = eval_in_context(context.borrow(), expr);

                if !result.is_none() {
                    render = true;
                }

                result
            },
            _ => eval_in_context(context.borrow(), expr),
        }
    }).collect::<Vec<EvalResult>>();

    if render {
        EvalResult::Vec(results)
    } else {
        EvalResult::None
    }
}
