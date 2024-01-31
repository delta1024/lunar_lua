use core::{panic, str};
use std::{ffi::{CString, CStr}, ptr::NonNull, slice};

use crate::{
    error::Result,
    raw_lua::{
        luaL_loadbufferx, luaL_newstate, luaL_openlibs, lua_State, lua_close, lua_gettop,
         lua_settop, LUA_OK, lua_tolstring, lua_pcallk,
    },
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
    pub fn pcall(&mut self, nargs: u32, nresults: u32, msgh: u32) -> Result<()> {
        let r = unsafe { lua_pcallk(self.0.as_mut(), nargs as i32, nresults as i32, msgh as i32, 0, None) };
        if r != 0 {
            return Err(r.into());
        }
        Ok(())
    }
    /// Pops n elements from the stack.
    pub fn pop(&mut self, n: u32) {
        self.set_op(self.get_op() - n);
    }
    ///  Accepts any index, or 0, and sets the stack top to this index. If the new top is greater than the old one,
    ///  then the new elements are filled with nil. If index is 0, then all stack elements are removed.
    ///
    /// This function can run arbitrary code when removing an index marked as to-be-closed from the stack.
    pub fn set_op(&mut self, idx: u32) {
        unsafe {
            lua_settop(self.0.as_mut(), idx as i32);
        }
    }
    pub fn to_string(&self, idx: i32) -> &str {
        let ptr: *const lua_State = unsafe {
           self.0.as_ref() 
        }; 
        let ptr = unsafe {
            lua_tolstring(ptr.cast_mut(), idx, std::ptr::null_mut())
        };
        
        let c_str = unsafe {
            CStr::from_ptr(ptr)
        };
        c_str.to_str().unwrap()
    }
    ///  Returns the index of the top element in the stack. Because indices start at 1, this result is equal to the number of elements in the stack; in particular, 0 means an empty stack.
    pub fn get_op(&self) -> u32 {
        unsafe {
            let ptr: *const lua_State = self.0.as_ref();
            lua_gettop(ptr.cast_mut()) as u32
        }
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
