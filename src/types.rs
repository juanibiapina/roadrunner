extern crate rlua;

use std::io;

#[derive(Debug)]
pub enum Error {
    ScriptError(rlua::Error),
    IoError(io::Error),
}

impl From<rlua::Error> for Error {
    fn from(err: rlua::Error) -> Error {
        Error::ScriptError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}
