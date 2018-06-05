extern crate termion;

use contexts::top_level::TopLevelContext;
use contexts::git::GitContext;
use contexts::fg::FgContext;
use contexts::bg::BgContext;
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
        Expr::Literal(literal) => EvalResult::Some(literal.0.to_string()),
        Expr::Placeholder(placeholder) => context.eval(placeholder.0),
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
                .map(|expr| eval_in_context(&context, expr))
                .collect::<Vec<EvalResult>>()
            )
        },
    }
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
