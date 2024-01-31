use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use crate::{
    error::Result,
    raw_lua::{
        luaL_loadbufferx, luaL_newstate, luaL_openlibs, lua_State, lua_checkstack, lua_close,
        lua_gettop, lua_pcallk, lua_settop, lua_tolstring, LUA_OK, lua_rawlen, lua_toboolean, lua_tonumberx, LUA_TNONE, LUA_TNIL, lua_type, LUA_TNUMBER, LUA_TBOOLEAN, LUA_TSTRING, LUA_TTABLE, LUA_TFUNCTION, LUA_TUSERDATA, LUA_TTHREAD, LUA_TLIGHTUSERDATA, lua_pushnil, lua_pushboolean, lua_pushnumber, lua_pushstring, lua_pushvalue, lua_rotate, lua_copy, lua_typename,
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
pub enum LuaType {
    None,
    Nil,
    Number,
    Boolean,
    String,
    Table,
    Function,
    UserData,
    Thread,
    LightUserData,
}
impl From<u32> for LuaType {
    fn from(value: u32) -> Self {
        match value {
            LUA_TNIL => Self::Nil,
            LUA_TNUMBER => Self::Number,
            LUA_TBOOLEAN => Self::Boolean,
            LUA_TSTRING => Self::String,
            LUA_TTABLE => Self::Table,
            LUA_TFUNCTION => Self::Function,
            LUA_TUSERDATA => Self::UserData,
            LUA_TTHREAD => Self::Thread,
            LUA_TLIGHTUSERDATA => Self::LightUserData,
            _ => unreachable!(),
        }
    }
}
pub enum StackValue<'a> {
    Nil,
    Bool(bool),
    Number(f64),
    String(&'a str),
}
impl From<f64> for StackValue<'_> {
    fn from(value: f64) -> Self {
        StackValue::Number(value.into())
    }
}
impl From<bool> for StackValue<'_> {
    fn from(value: bool) -> Self {
        StackValue::Bool(value)
    }
}
impl<'a> From<&'a str> for StackValue<'a> {
    fn from(value: &'a str) -> Self {
        Self::String(value)
    }
}
impl From<Option<()>> for StackValue<'_> {
    fn from(_: Option<()>) -> Self {
        StackValue::Nil
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
    pub fn push<'a, T: Into<StackValue<'a>>>(&mut self, val: T) {
        match val.into() {
            StackValue::Nil => unsafe {
                lua_pushnil(self.0.as_ptr());
            },
            StackValue::Bool(b) => unsafe {
                lua_pushboolean(self.0.as_ptr(), b as i32);
            },
            StackValue::Number(n) => unsafe {
                lua_pushnumber(self.0.as_ptr(), n);
            },
            StackValue::String(s) => unsafe {
                let c_string = CString::new(s).expect("oom");
                lua_pushstring(self.0.as_ptr(), c_string.as_ptr());
            },
        }
    }

    /// Ensures that the stack has space for at least n extra elements, that is, that you can safely push up to n values into it.
    /// It returns false if it cannot fulfill the request, either because it would cause the stack to be greater than a fixed maximum size 
    /// (typically at least several thousand elements) or because it cannot allocate memory for the extra space. 
    /// This function never shrinks the stack; if the stack already has space for the extra elements, it is left unchanged. 
    pub fn check_stack(&self, sz: i32) -> i32 {
        unsafe {
            let ptr: *const lua_State = self.0.as_ref();
            lua_checkstack(ptr.cast_mut(), sz)
        }
    }
    pub fn pcall(&mut self, nargs: u32, nresults: u32, msgh: u32) -> Result<()> {
        let r = unsafe {
            lua_pcallk(
                self.0.as_mut(),
                nargs as i32,
                nresults as i32,
                msgh as i32,
                0,
                None,
            )
        };
        if r != 0 {
            return Err(r.into());
        }
        Ok(())
    }
    /// Pops n elements from the stack.
    pub fn pop(&mut self, n: i32) {
        self.set_top(-n - 1);
    }
    ///  Accepts any index, or 0, and sets the stack top to this index. If the new top is greater than the old one,
    ///  then the new elements are filled with nil. If index is 0, then all stack elements are removed.
    ///
    /// This function can run arbitrary code when removing an index marked as to-be-closed from the stack.
    pub fn set_top(&mut self, idx: i32) {
        unsafe {
            lua_settop(self.0.as_mut(), idx);
        }
    }
    pub fn push_value(&mut self, idx: i32) {
        unsafe {
            lua_pushvalue(self.0.as_ptr(), idx);
        }
    }
    pub fn remove(&mut self, idx: i32) {
        unsafe {
            lua_rotate(self.0.as_ptr(), idx, -1);
            self.pop(1);
        }
    }
    pub fn insert(&mut self, idx: i32) {
        unsafe {
            lua_rotate(self.0.as_ptr(), idx, 1);
        }
    }
    pub fn replace(&mut self, idx: i32) {
        unsafe {
            lua_copy(self.0.as_ptr(), -1, idx);
            self.pop(1);
        }
    }   
    pub fn get_type(&mut self, idx: i32) -> LuaType {
        let ty = unsafe {
            let ptr = self.0.as_ptr();
            lua_type(ptr, idx)
        };
        match ty {
            LUA_TNONE => LuaType::None,
            _ => (ty as u32).into(),
        }
    }
    pub fn type_name(&self, idx: i32) -> &str {
        unsafe {
            let ptr = self.0.as_ref() as *const lua_State;
            let ptr = lua_typename(ptr.cast_mut(), idx);
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
    pub fn to_string(&self, idx: i32) -> Option<&str> {
        let c_str = unsafe {
            let ptr: *const lua_State = self.0.as_ref();
            let ptr = lua_tolstring(ptr.cast_mut(), idx, std::ptr::null_mut());
            if ptr.is_null() {
                return None;
            }
            CStr::from_ptr(ptr)
        };
        c_str.to_str().unwrap().into()
    }
    pub fn to_boolean(&self, idx: i32) -> bool {
        unsafe {
            let ptr = self.0.as_ref() as *const lua_State;
            match lua_toboolean(ptr.cast_mut(), idx) {
                0 => false,
                _ => true,
            }
        }
    }
    pub fn to_number(&self, idx: i32) -> f64 {
        unsafe {
            let ptr = self.0.as_ref() as *const lua_State;
            lua_tonumberx(ptr.cast_mut(), idx, std::ptr::null_mut())
        }
    }
    pub fn strlen(&self, idx: i32) -> u64 {
        unsafe {
            let ptr = self.0.as_ref() as *const lua_State;
            lua_rawlen(ptr.cast_mut(), idx)
        }
    }
    ///  Returns the index of the top element in the stack. Because indices start at 1, this result is equal to the number of elements in the stack; in particular, 0 means an empty stack.
    pub fn get_top(&self) -> u32 {
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
