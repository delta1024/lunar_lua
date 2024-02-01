use std::ffi::CString;

use crate::{
    ffi::{
        luaL_requiref, luaopen_base, luaopen_coroutine, luaopen_debug, luaopen_io, luaopen_math,
        luaopen_os, luaopen_package, luaopen_string, luaopen_table, luaopen_utf8,
    },
    LuaConn,
};

impl<T: LuaConn> LuaStandardLib for T {}
pub trait LuaStandardLib: LuaConn {
    fn open_base(&self) {
        let mod_name = CString::new("base").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_base),
                1,
            );
        }
    }
    fn open_coroutine(&self) {
        let mod_name = CString::new("coroutine").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_coroutine),
                1,
            );
        }
    }
    fn open_debug(&self) {
        let mod_name = CString::new("debug").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_debug),
                1,
            );
        }
    }
    fn open_io(&self) {
        let mod_name = CString::new("io").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_io),
                1,
            );
        }
    }
    fn open_math(&self) {
        let mod_name = CString::new("math").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_math),
                1,
            );
        }
    }
    fn open_os(&self) {
        let mod_name = CString::new("os").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_os),
                1,
            );
        }
    }
    fn open_package(&self) {
        let mod_name = CString::new("package").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_package),
                1,
            );
        }
    }
    fn open_string(&self) {
        let mod_name = CString::new("string").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_string),
                1,
            );
        }
    }
    fn open_table(&self) {
        let mod_name = CString::new("table").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_table),
                1,
            );
        }
    }
    fn open_utf8(&self) {
        let mod_name = CString::new("utf8").unwrap();
        unsafe {
            luaL_requiref(
                self.get_conn().get_mut_ptr(),
                mod_name.as_ptr(),
                Some(luaopen_utf8),
                1,
            );
        }
    }
}
