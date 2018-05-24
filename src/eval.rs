extern crate git2;

use self::git2::Repository;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use types::*;
use utils;

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
        "git" => {
            let repo = match Repository::open(".") {
                Ok(repo) => repo,
                Err(_) => return "".to_owned(),
            };

            section.exprs.iter().map(|expr| eval_git(&repo, expr)).collect::<Vec<String>>().join("")
        },
        "rbenv" => {
            let version_file = Path::new(".ruby-version");
            if ! version_file.exists() {
                return "".to_owned();
            }

            section.exprs.iter().map(eval_rbenv).collect::<Vec<String>>().join("")
        },
        _ => panic!("unsupported section"),
    }
}

fn eval_top_level_placeholder(name: &str) -> String {
    match name {
        "hostname" => utils::hostname(),
        "username" => utils::username(),
        "cwd" => utils::cwd(),
        _ => panic!("unsupported placeholder"),
    }
}

fn eval_git(repo: &Repository, expr: &Expr) -> String {
    match expr {
        Expr::Literal(value) => value.to_string(),
        Expr::Placeholder(name) => {
            match name {
                &"branch" => {
                    repo.head().unwrap().shorthand().unwrap().to_string()
                },
                _ => panic!("unsupported integration placeholder"),
            }
        },
    }
}

fn eval_rbenv(expr: &Expr) -> String {
    match expr {
        Expr::Literal(value) => value.to_string(),
        Expr::Placeholder(name) => {
            match name {
                &"version" => {
                    let version_file = Path::new(".ruby-version");
                    let mut file = File::open(version_file).unwrap();

                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    contents.trim().to_string()
                },
                _ => panic!("unsupported integration placeholder"),
            }
        },
    }
}
