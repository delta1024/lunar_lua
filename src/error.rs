use std::{error, fmt::Display, result};

use crate::raw_lua::{LUA_ERRERR, LUA_ERRFILE, LUA_ERRMEM, LUA_ERRRUN, LUA_ERRSYNTAX};

#[derive(Debug)]
pub enum Error {
    /// a runtime error.
    Run,
    /// memory allocation error. For such errors, Lua does not call the message handler.
    Mem,
    /// error while running the message handler.
    Err,
    /// syntax error during precompilation.
    Syntax,
    /// a file-related error; e.g., it cannot open or read the file.
    File,
}

impl From<i32> for Error {
    fn from(value: i32) -> Self {
        match value as u32 {
            LUA_ERRRUN => Self::Run,
            LUA_ERRMEM => Self::Mem,
            LUA_ERRERR => Self::Err,
            LUA_ERRSYNTAX => Self::Syntax,
            LUA_ERRFILE => Self::File,
            _ => unreachable!(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    // add code here
}

pub type Result<T> = result::Result<T, Error>;
