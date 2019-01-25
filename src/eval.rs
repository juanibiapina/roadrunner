use types::*;
use context::Context;
use contexts::rbenv;
use functions;

pub fn eval(prompt: &Prompt) -> String {
    let top_level_context = Context::top_level();

    prompt.sections
        .iter()
        .filter_map(|section| eval_section(&top_level_context, section))
        .map(|rendered_section| rendered_section.content)
        .collect::<Vec<String>>()
        .join("")
}

fn eval_section(context: &Context, section: &Section) -> Option<RenderedSection> {
    match section.name {
        Some(ref name) => {
            match name.as_ref() {
                "rbenv" => {
                    match rbenv::init(context) {
                        Some(context) => {
                            Some(RenderedSection {
                                content: section.parts
                                            .iter()
                                            .filter_map(|part| eval_part(&context, part))
                                            .collect::<Vec<_>>()
                                            .join("")
                            })
                        },
                        None => None,
                    }
                },
                _ => panic!("Unknown section name"),
            }
        },
        None => {
            Some(RenderedSection {
                content: section.parts
                            .iter()
                            .filter_map(|part| eval_part(context, part))
                            .collect::<Vec<_>>()
                            .join("")
            })
        },
    }
}

fn eval_part(context: &Context, part: &Part) -> Option<String> {
    match part {
        Part::Literal(value) => Some(value.to_owned()),
        Part::Interpolation(expr) => {
            let evaluated = eval_expr(context, expr);
            match evaluated {
                Expr::String(value) => Some(value),
                _ => None,
            }
        },
    }
}

fn eval_expr(context: &Context, expr: &Expr) -> Expr {
    match expr {
        Expr::FunctionCall(name, args) => {
            let evaluated_args = args.iter().map(|arg| eval_expr(context, arg)).collect::<Vec<_>>();
            invoke_function(name, evaluated_args.as_slice())
        },
        Expr::Variable(name) => context.get(name),
        Expr::String(value) => Expr::String(value.to_owned()),
    }
}

fn invoke_function(name: &str, args: &[Expr]) -> Expr {
    match name {
        "cwd" => Expr::String(functions::cwd()),
        "hostname" => Expr::String(functions::hostname()),
        "username" => Expr::String(functions::username()),
        "fg" => functions::fg(args),
        "bg" => functions::bg(args),
        _ => Expr::String("".to_owned()),
    }
}
