extern crate rlua;

use self::rlua::Lua;

use types::Error;
use utils;
use runtime::rbenv::Rbenv;
use runtime::git::Git;
use runtime::colors::{self, Color, ColorName};

pub struct Engine {
    lua: Lua,
}

impl Engine {
    pub fn new() -> Engine {
        let lua = Lua::new();

        {
            let globals = lua.globals();

            globals.set("username", lua.create_function(|_, ()| { Ok(utils::username()) }).unwrap()).unwrap();
            globals.set("hostname", lua.create_function(|_, ()| { Ok(utils::hostname()) }).unwrap()).unwrap();
            globals.set("cwd", lua.create_function(|_, ()| { Ok(utils::cwd()) }).unwrap()).unwrap();

            globals.set("fg", lua.create_function(|_, c: Color| { Ok(colors::fg(c)) }).unwrap()).unwrap();
            globals.set("bg", lua.create_function(|_, c: Color| { Ok(colors::bg(c)) }).unwrap()).unwrap();

            globals.set("black", Color::Name(ColorName::Black)).unwrap();
            globals.set("blue", Color::Name(ColorName::Blue)).unwrap();
            globals.set("cyan", Color::Name(ColorName::Cyan)).unwrap();
            globals.set("green", Color::Name(ColorName::Green)).unwrap();
            globals.set("light_black", Color::Name(ColorName::LightBlack)).unwrap();
            globals.set("light_blue", Color::Name(ColorName::LightBlue)).unwrap();
            globals.set("light_cyan", Color::Name(ColorName::LightCyan)).unwrap();
            globals.set("light_green", Color::Name(ColorName::LightGreen)).unwrap();
            globals.set("light_magenta", Color::Name(ColorName::LightMagenta)).unwrap();
            globals.set("light_red", Color::Name(ColorName::LightRed)).unwrap();
            globals.set("light_white", Color::Name(ColorName::LightWhite)).unwrap();
            globals.set("light_yellow", Color::Name(ColorName::LightYellow)).unwrap();
            globals.set("magenta", Color::Name(ColorName::Magenta)).unwrap();
            globals.set("red", Color::Name(ColorName::Red)).unwrap();
            globals.set("red", Color::Name(ColorName::Red)).unwrap();
            globals.set("white", Color::Name(ColorName::White)).unwrap();
            globals.set("yellow", Color::Name(ColorName::Yellow)).unwrap();

            globals.set("reset", Color::Reset).unwrap();

            globals.set("rbenv_init", lua.create_function(|_, ()| { Ok(Rbenv::new()) }).unwrap()).unwrap();
            globals.set("git_init", lua.create_function(|_, ()| { Ok(Git::new()) }).unwrap()).unwrap();
        }

        Engine {
            lua,
        }
    }

    pub fn run(&mut self, input: &str) -> Result<String, Error> {
        Ok(self.lua.eval::<String>(input, None)?)
    }

    pub fn run_file(&mut self, filename: &str) -> Result<String, Error> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(filename)?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;

        self.run(&contents)
    }
}
