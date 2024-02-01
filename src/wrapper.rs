use std::{ffi::CStr, mem, ptr::NonNull};

use crate::{
    ffi::{
        luaL_newstate, lua_State, lua_close, lua_pcallk, lua_settop, lua_tolstring, lua_type,
        lua_typename, LUA_OK,
    },
    LuaConn,
};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LuaError {
    Run = crate::ffi::LUA_ERRRUN as i32,
    Mem = crate::ffi::LUA_ERRMEM as i32,
    Err = crate::ffi::LUA_ERRERR as i32,
    Syntax = crate::ffi::LUA_ERRSYNTAX as i32,
    File = crate::ffi::LUA_ERRFILE as i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LuaType {
    None = crate::ffi::LUA_TNONE,
    Nil = crate::ffi::LUA_TNIL as i32,
    Number = crate::ffi::LUA_TNUMBER as i32,
    Boolean = crate::ffi::LUA_TBOOLEAN as i32,
    String = crate::ffi::LUA_TSTRING as i32,
    Table = crate::ffi::LUA_TTABLE as i32,
    Function = crate::ffi::LUA_TFUNCTION as i32,
    UserData = crate::ffi::LUA_TUSERDATA as i32,
    Thread = crate::ffi::LUA_TTHREAD as i32,
    LightUserData = crate::ffi::LUA_TLIGHTUSERDATA as i32,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct State(NonNull<lua_State>);

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            lua_close(self.get_conn().get_mut_ptr());
        }
    }
}
unsafe impl LuaConn for State {
    fn get_conn(&self) -> crate::LuaConnection<'_> {
        unsafe { crate::LuaConnection(self.0.as_ref()) }
    }
}
impl State {
    pub fn new() -> State {
        let ptr = unsafe {
            let ptr = luaL_newstate();
            if ptr.is_null() {
                panic!("Could not aquire lua state");
            }
            NonNull::new_unchecked(ptr)
        };
        State(ptr)
    }
}
