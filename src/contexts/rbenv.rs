use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use types::Context;
use types::EvalResult;

pub struct RbenvContext {
    version_file: PathBuf,
}

impl RbenvContext {
    pub fn new() -> Option<RbenvContext> {
        let path = env::current_dir().unwrap();

        for path in path.ancestors() {
            let mut version_file = PathBuf::from(path);
            version_file.push(".ruby-version");

            if version_file.exists() {
                return Some(RbenvContext {
                    version_file: version_file,
                });
            }
        }

        None
    }
}

impl Context for RbenvContext {
    fn eval(&self, name: &str) -> EvalResult {
        match name {
            "version" => {
                let mut file = File::open(&self.version_file).unwrap();

                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                EvalResult::Some(contents.trim().to_string())
            },
            _ => EvalResult::None,
        }
    }
}


