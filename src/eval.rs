use types::*;
use functions;

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
        "cwd" => Expr::String(functions::cwd()),
        "hostname" => Expr::String(functions::hostname()),
        "username" => Expr::String(functions::username()),
        "fg" => functions::fg(args),
        "bg" => functions::bg(args),
        _ => Expr::String("".to_owned()),
    }
}
