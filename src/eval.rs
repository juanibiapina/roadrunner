use types::*;
use context::Context;
use contexts::rbenv;
use contexts::git;
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
                                            .map(|p| p.content)
                                            .collect::<Vec<_>>()
                                            .join("")
                            })
                        },
                        None => None,
                    }
                },
                "git" => {
                    match git::init(context) {
                        Some(context) => {
                            Some(RenderedSection {
                                content: section.parts
                                            .iter()
                                            .filter_map(|part| eval_part(&context, part))
                                            .map(|p| p.content)
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
                            .map(|p| p.content)
                            .collect::<Vec<_>>()
                            .join("")
            })
        },
    }
}

fn eval_part(context: &Context, part: &Part) -> Option<RenderedPart> {
    match part {
        Part::Literal(value) => Some(RenderedPart {
            content: value.to_owned(),
            trigger: false,
        }),
        Part::Interpolation(expr) => {
            let evaluated = eval_expr(context, expr);
            match evaluated {
                Expr::String(value) => Some(RenderedPart {
                    content: value,
                    trigger: false,
                }),
                Expr::Trigger(value) => Some(RenderedPart {
                    content: value,
                    trigger: true,
                }),
                _ => panic!("Unexpected interpolation result"),
            }
        },
        Part::Conditional(parts) => {
            let rendered_parts = parts
                .iter()
                .filter_map(|part| eval_part(context, part))
                .collect::<Vec<RenderedPart>>();

            let render = rendered_parts.iter().fold(false, |acc, rendered_part| acc || rendered_part.trigger);

            if render {
                Some(RenderedPart {
                    content: rendered_parts.into_iter().map(|p| p.content).collect::<Vec<String>>().join(""),
                    trigger: true,
                })
            } else {
                None
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
        Expr::Trigger(_) => expr.clone(),
        Expr::String(_) => expr.clone(),
        Expr::Number(_) => expr.clone(),
        Expr::Boolean(_) => expr.clone(),
    }
}

fn invoke_function(name: &str, args: &[Expr]) -> Expr {
    match name {
        "cwd" => Expr::String(functions::cwd()),
        "hostname" => Expr::String(functions::hostname()),
        "username" => Expr::String(functions::username()),
        "fg" => functions::fg(args),
        "bg" => functions::bg(args),
        "tr" => functions::tr(args),
        _ => panic!("Unknown function: `{}`", name),
    }
}
