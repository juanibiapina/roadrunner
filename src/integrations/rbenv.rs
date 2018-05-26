use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use types::Integration;
use types::Placeholder;

pub struct Rbenv {
    version_file: PathBuf,
}

impl Rbenv {
    pub fn new() -> Option<Rbenv> {
        let path = env::current_dir().unwrap();

        for path in path.ancestors() {
            let mut version_file = PathBuf::from(path);
            version_file.push(".ruby-version");

            if version_file.exists() {
                return Some(Rbenv {
                    version_file: version_file,
                });
            }
        }

        None
    }
}

impl Integration for Rbenv {
    fn eval(&self, placeholder: &Placeholder) -> String {
        match placeholder.0 {
            "version" => {
                let mut file = File::open(&self.version_file).unwrap();

                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                contents.trim().to_string()
            },
            _ => panic!("unsupported integration placeholder"),
        }
    }
}


