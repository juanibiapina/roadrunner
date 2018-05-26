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
        let mut version_file = PathBuf::new();
        version_file.push(".ruby-version");

        if version_file.exists() {
            Some(Rbenv {
                version_file: version_file,
            })
        } else {
            None
        }
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


