use expr::*;

pub fn eval(expr: &Expr) -> String {
    match expr {
        Expr::Prompt(expr) => expr.iter().map(eval).collect::<Vec<String>>().join(""),
        Expr::Literal(value) => value.to_string(),
        Expr::Section(expr) => eval(expr),
        Expr::TaggedSpec(tag, expr) => eval_spec(tag, expr),
        _ => panic!("invariant violated"),
    }
}

fn eval_spec(tag: &str, expr: &Expr) -> String {
    match expr {
        Expr::Spec(_) => {
            match tag {
                "git" => "git".to_owned(),
                "rbenv" => "rbenv".to_owned(),
                _ => "unsupported_tag".to_owned(),
            }
        },
        _ => panic!("invariant violated"),
    }
}
