extern crate rlua;

use self::rlua::{UserData, UserDataMethods};

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Rbenv {
    version_file: Option<PathBuf>,
}

impl UserData for Rbenv {
    fn add_methods(methods: &mut UserDataMethods<Self>) {
        methods.add_method("enabled", |_, rbenv, ()| {
            Ok(rbenv.enabled())
        });

        methods.add_method("version", |_, rbenv, ()| {
            Ok(rbenv.version())
        });
    }
}

impl Rbenv {
    pub fn new() -> Rbenv {
        let path = env::current_dir().unwrap();

        for path in path.ancestors() {
            let mut version_file = PathBuf::from(path);
            version_file.push(".ruby-version");

            if version_file.exists() {
                return Rbenv {
                    version_file: Some(version_file),
                };
            }
        }

        Rbenv {
            version_file: None,
        }
    }

    pub fn enabled(&self) -> bool {
        self.version_file.is_some()
    }

    pub fn version(&self) -> String {
        match self.version_file {
            Some(ref version_file) => {
                let mut file = File::open(version_file).unwrap();

                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                contents.trim().to_string()
            },
            None => "".to_owned(),
        }
    }
}
