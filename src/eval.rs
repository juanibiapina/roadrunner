use types::*;

pub fn eval(prompt: &Prompt) -> String {
    prompt.exprs.iter().map(eval_top_level_expr).collect::<Vec<String>>().join("")
}

fn eval_top_level_expr(expr: &TopLevelExpr) -> String {
    match expr {
        TopLevelExpr::Expr(expr) => {
            match expr {
                Expr::Literal(value) => value.to_string(),
                Expr::Placeholder(name) => eval_top_level_placeholder(name),
            }
        },
        TopLevelExpr::Section(value) => {
            eval_section(value)
        }
    }
}

fn eval_section(section: &Section) -> String {
    match section.name {
        "git" => "git".to_owned(),
        "rbenv" => "rbenv".to_owned(),
        _ => panic!("unsupported section"),
    }
}

fn eval_top_level_placeholder(name: &str) -> String {
    name.to_string()
}
