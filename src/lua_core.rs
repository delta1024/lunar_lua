use core::str;
use std::{
    ffi::{CStr, CString},
    mem,
};

use crate::{
    ffi::{
        lua_checkstack, lua_copy, lua_gettop, lua_pcallk, lua_pushboolean, lua_pushlstring,
        lua_pushnil, lua_pushnumber, lua_pushstring, lua_pushvalue, lua_rotate, lua_settop,
        lua_toboolean, lua_tolstring, lua_tonumberx, lua_type, lua_typename, LUA_OK,
    },
    LuaConn, LuaError, LuaType,
};

macro_rules! check_for_err {
    ($result:tt) => {
        if $result != LUA_OK as i32 {
            return Err(unsafe { std::mem::transmute($result) });
        }
    };
}
pub enum LuaStackValue<'a> {
    Number(f64),
    Bool(bool),
    String(&'a str),
    Nil,
}
impl From<f64> for LuaStackValue<'_> {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl From<bool> for LuaStackValue<'_> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}
impl<'a> From<&'a str> for LuaStackValue<'a> {
    fn from(value: &'a str) -> Self {
        Self::String(value)
    }
}
impl From<Option<()>> for LuaStackValue<'_> {
    fn from(_: Option<()>) -> Self {
        Self::Nil
    }
}
pub trait LuaCore: LuaConn {
    fn pcall(&self, nargs: i32, nresults: i32, msgh: i32) -> Result<(), LuaError> {
        let result = unsafe {
            lua_pcallk(
                self.get_conn().get_mut_ptr(),
                nargs,
                nresults,
                msgh,
                0,
                None,
            )
        };
        check_for_err!(result);
        Ok(())
    }
    fn get_type(&self, index: i32) -> LuaType {
        unsafe {
            let ptr = self.get_conn().get_mut_ptr();
            let ty = lua_type(ptr, index);
            mem::transmute(ty)
        }
    }
    fn type_name(&self, index: i32) -> &str {
        unsafe {
            let ptr = self.get_conn().get_mut_ptr();
            let ptr = lua_typename(ptr, index);
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
    fn pop(&self, n: i32) {
        self.set_top(-n - 1);
    }
    fn set_top(&self, idx: i32) {
        unsafe {
            lua_settop(self.get_conn().get_mut_ptr(), idx);
        }
    }

    fn get_top(&self) -> i32 {
        unsafe {
            let ptr = self.get_conn().get_mut_ptr();
            lua_gettop(ptr)
        }
    }

    fn push_nil(&self) {
        unsafe {
            lua_pushnil(self.get_conn().get_mut_ptr());
        }
    }
    fn push_boolean(&self, b: bool) {
        unsafe {
            lua_pushboolean(self.get_conn().get_mut_ptr(), b as i32);
        }
    }
    fn push_number(&self, n: f64) {
        unsafe {
            lua_pushnumber(self.get_conn().get_mut_ptr(), n);
        }
    }
    fn push_l_string<'a>(&'a self, s: &str, length: isize) -> &'a str {
        let c_string = CString::new(s).unwrap();
        unsafe {
            CStr::from_ptr(lua_pushlstring(
                self.get_conn().get_mut_ptr(),
                c_string.as_ptr(),
                length as usize,
            ))
            .to_str()
            .unwrap()
        }
    }
    fn push_string<'a>(&'a self, s: &str) -> &'a str {
        let c_string = CString::new(s).unwrap();
        unsafe {
            CStr::from_ptr(lua_pushstring(
                self.get_conn().get_mut_ptr(),
                c_string.as_ptr(),
            ))
            .to_str()
            .unwrap()
        }
    }
    fn push_value(&self, idx: i32) {
        unsafe {
            lua_pushvalue(self.get_conn().get_mut_ptr(), idx);
        }
    }
    fn push<'a, 'lua, T: Into<LuaStackValue<'a>>>(&'lua self, value: T) -> Option<&'lua str> {
        match value.into() {
            LuaStackValue::Nil => {
                self.push_nil();
                None
            }
            LuaStackValue::Bool(b) => {
                self.push_boolean(b);
                None
            }
            LuaStackValue::Number(n) => {
                self.push_number(n);
                None
            }
            LuaStackValue::String(s) => Some(self.push_string(s)),
        }
    }
    fn is_nil(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::Nil)
    }
    fn is_boolean(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::Boolean)
    }
    fn is_number(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::Number)
    }

    fn is_string(&self, idx: i32) -> bool {
        matches!(self.get_type(idx), LuaType::String)
    }

    fn to_string(&self, idx: i32) -> &str {
        unsafe {
            let ptr = self.get_conn().get_mut_ptr();
            let ptr = lua_tolstring(ptr, idx, std::ptr::null_mut());
            CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
    fn to_number(&self, idx: i32) -> f64 {
        unsafe { lua_tonumberx(self.get_conn().get_mut_ptr(), idx, std::ptr::null_mut()) }
    }

    fn to_boolean(&self, idx: i32) -> bool {
        unsafe {
            match lua_toboolean(self.get_conn().get_mut_ptr(), idx) {
                0 => false,
                _ => true,
            }
        }
    }
    fn check_stack(&self, sz: i32) -> bool {
        unsafe {
            match lua_checkstack(self.get_conn().get_mut_ptr(), sz) {
                0 => false,
                _ => true,
            }
        }
    }
    fn remove(&self, idx: i32) {
        self.rotate(idx, -1);
        self.pop(1)
    }

    fn insert(&self, idx: i32) {
        self.rotate(idx, 1);
    }

    fn replace(&self, idx: i32) {
        self.copy(-1, idx);
        self.pop(1);
    }

    fn copy(&self, from_idx: i32, to_idx: i32) {
        unsafe {
            lua_copy(self.get_conn().get_mut_ptr(), from_idx, to_idx);
        }
    }
    fn rotate(&self, idx: i32, n: i32) {
        unsafe {
            lua_rotate(self.get_conn().get_mut_ptr(), idx, n);
        }
    }
}
impl<T: LuaConn> LuaCore for T {}
