use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use context::Context;
use types::Expr;

pub fn init<'a>(parent: &'a Context) -> Option<Context<'a>> {
    let path = env::current_dir().unwrap();

    for path in path.ancestors() {
        let mut version_file = PathBuf::from(path);
        version_file.push(".ruby-version");

        if version_file.exists() {
            let version = version(version_file);

            let mut context = Context::new(Some(parent));
            context.set("version", &Expr::String(version));

            return Some(context);
        }
    }

    None
}

fn version(version_file: PathBuf) -> String {
    let mut file = File::open(&version_file).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.trim().to_string()
}
