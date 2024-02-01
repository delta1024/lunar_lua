use std::{ffi::CStr, mem};

use crate::{
    ffi::{lua_gettop, lua_pcallk, lua_settop, lua_tolstring, lua_type, lua_typename, LUA_OK},
    LuaConn, LuaError, LuaType,
};

macro_rules! check_for_err {
    ($result:tt) => {
        if $result != LUA_OK as i32 {
            return Err(unsafe { std::mem::transmute($result) });
        }
    };
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
    fn to_string(&self, idx: i32) -> &str {
        unsafe {
            let ptr = self.get_conn().get_mut_ptr();
            let ptr = lua_tolstring(ptr, idx, std::ptr::null_mut());
            CStr::from_ptr(ptr).to_str().unwrap()
        }
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
    
}
impl<T: LuaConn> LuaCore for T {}
