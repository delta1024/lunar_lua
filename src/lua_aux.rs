use std::ffi::CString;

use crate::{
    ffi::{luaL_loadbufferx, luaL_openlibs, LUA_OK, lua_State, luaL_newstate},
    wrapper::LuaError,
    LuaConn,
};

pub fn aux_new_state() -> *mut lua_State {
    unsafe {
        luaL_newstate()
    }
}
pub trait LuaAuxLib: LuaConn {
    fn aux_load_buffer(&self, buff: &str, name: &str) -> Result<(), LuaError> {
        let (buff, name) = (
            CString::new(buff).expect("invalid string"),
            CString::new(name).expect("invalid string"),
        );
        let result = unsafe {
            luaL_loadbufferx(
                self.get_conn().get_mut_ptr(),
                buff.as_ptr(),
                buff.as_bytes().len(),
                name.as_ptr(),
                std::ptr::null_mut(),
            )
        };
        if result != LUA_OK as i32 {
            return Err(unsafe { std::mem::transmute(result) });
        }
        Ok(())
    }
    fn aux_open_libs(&self) {
        unsafe {
            luaL_openlibs(self.get_conn().get_mut_ptr());
        }
    }
}
impl<T: LuaConn> LuaAuxLib for T {}
