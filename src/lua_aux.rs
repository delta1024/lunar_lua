use std::ffi::{CStr, CString};

use crate::{
    check_for_err,
    ffi::{
        luaL_checklstring, luaL_checknumber, luaL_error, luaL_loadbufferx, luaL_loadfilex,
        luaL_newstate, luaL_openlibs, lua_State, LUA_OK,
    },
    wrapper::LuaError,
    LuaConn,
};
/// Formats and reports an error. Calls [aux_error](LuaAuxLib::aux_error).
#[macro_export]
macro_rules! lua_error {
    ($self:ident, $($fmt:tt)*) => {
        {
        use $crate::LuaAuxLib;
        $self.aux_error(format!($($fmt)*))
        }
    };
}
/// Creates a new Lua state. It calls [crate::lua_core::new_state] with an allocator based on the ISO C allocation functions and then sets a warning function and a panic function (see §4.4) that print messages to the standard error output.
pub fn aux_new_state() -> *mut lua_State {
    unsafe { luaL_newstate() }
}
pub trait LuaAuxLib: LuaConn {
    /// Equivalent to [luaL_loadbufferx] with mode equal to NULL.
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
        check_for_err!(result);
        Ok(())
    }
    /// Equivalent to [luaL_loadfilex] with mode equal to NULL.
    fn aux_load_file(&self, file_name: &str) -> Result<(), LuaError> {
        let c_str = CString::new(file_name).unwrap();
        let result = unsafe {
            luaL_loadfilex(
                self.get_conn().get_mut_ptr(),
                c_str.as_ptr(),
                std::ptr::null(),
            )
        };
        check_for_err!(result);
        Ok(())
    }
    /// Opens all standard Lua libraries into the given state.  
    fn aux_open_libs(&self) {
        unsafe {
            luaL_openlibs(self.get_conn().get_mut_ptr());
        }
    }
    /// Checks whether the function argument arg is a number and returns this number converted to a lua_Number.
    fn aux_check_number(&self, arg: i32) -> f64 {
        unsafe { luaL_checknumber(self.get_conn().get_mut_ptr(), arg) }
    }
    ///  Checks whether the function argument arg is a string and returns this string.
    ///
    /// This function uses [crate::ffi::lua_tolstring] to get its result, so all conversions and caveats of that function apply here.
    fn aux_check_string(&self, arg: i32) -> &str {
        unsafe {
            CStr::from_ptr(luaL_checklstring(
                self.get_conn().get_mut_ptr(),
                arg,
                std::ptr::null_mut(),
            ))
            .to_str()
            .unwrap()
        }
    }
    ///  Raises an error. It also adds at the beginning of the message the file name and the line number where the error occurred, if this information is available.
    ///
    /// This function never returns, but it is an idiom to use it in C functions as return luaL_error(args).
    fn aux_error<T: ToString>(&self, message: T) {
        let message = CString::new(message.to_string()).unwrap();
        unsafe {
            luaL_error(self.get_conn().get_mut_ptr(), message.as_ptr());
        }
    }
}
impl<T: LuaConn> LuaAuxLib for T {}
