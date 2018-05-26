extern crate git2;

use self::git2::Repository;

use types::Integration;
use types::Placeholder;

pub struct Git {
    repo: Repository,
}

impl Git {
    pub fn new() -> Option<Git> {
        Repository::open(".").ok().map(|repo| Git { repo: repo })
    }
}

impl Integration for Git {
    fn eval(&self, placeholder: &Placeholder) -> String {
        match placeholder.0 {
            "branch" => {
                self.repo.head().unwrap().shorthand().unwrap().to_string()
            },
            _ => panic!("unsupported integration placeholder"),
        }
    }
}
