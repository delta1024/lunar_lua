use core::panic;
use std::{ffi::CString, ptr::NonNull};

use crate::{
    error::Result,
    raw_lua::{luaL_loadbufferx, luaL_newstate, luaL_openlibs, lua_State, lua_close, LUA_OK},
};

#[derive(Debug, Default)]
pub enum LoadMode {
    BinaryChunks,
    TextChunks,
    #[default]
    BinaryTextChunks,
}

impl LoadMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BinaryChunks => "b",
            Self::TextChunks => "t",
            Self::BinaryTextChunks => "bt",
        }
    }
}
pub struct State(NonNull<lua_State>);

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            lua_close(self.0.as_ptr());
        }
    }
}

impl State {
    ///  Creates a new Lua state.
    ///  # Panics:
    ///  * out of memory
    pub fn new() -> Self {
        let state = unsafe {
            let n = luaL_newstate();
            if n.is_null() {
                panic!("oom");
            }
            NonNull::new_unchecked(n)
        };
        State(state)
    }

    /// Opens all standard Lua libraries into the given state.
    pub fn open_libs(&mut self) {
        unsafe {
            luaL_openlibs(self.0.as_mut());
        }
    }
    /// Equivalent to [load_buffer_with_mode](State::load_buffer_with_mode) with mode equal to None.
    pub fn load_buffer(&mut self, buff: &str, name: &str) -> Result<()> {
        self.load_buffer_with_mode(buff.as_bytes(), name, None)
    }
    ///  Loads a buffer as a Lua chunk. This function uses lua_load to load the chunk in the buffer pointed to by buff
    /// # Params
    /// * name is the chunk name, used for debug information and error messages.
    pub fn load_buffer_with_mode(
        &mut self,
        buff: &[u8],
        name: &str,
        mode: Option<LoadMode>,
    ) -> Result<()> {
        let mode = match mode {
            Some(m) => CString::new(m.as_str()),
            None => CString::new(LoadMode::default().as_str()),
        }
        .expect("memerr");
        let buff = CString::new(buff).expect("memerr");
        let name = CString::new(name).expect("memerr");
        unsafe {
            let n = luaL_loadbufferx(
                self.0.as_mut(),
                buff.as_ptr(),
                buff.as_bytes().len(),
                name.as_ptr(),
                mode.as_ptr(),
            );
            if n != LUA_OK as i32 {
                return Err(n.into());
            }
        }
        Ok(())
    }
}
#[cfg(test)]
pub mod tests {
    use super::State;

    #[test]
    fn test_drop() {
        let mut lua = State::new();
        lua.open_libs();
        assert!(0 == 0);
    }
}
