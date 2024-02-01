use std::ffi::CString;

use crate::{
    ffi::{luaL_loadbufferx, luaL_newstate, luaL_openlibs, lua_State, LUA_OK, luaL_error, luaL_loadfilex},
    wrapper::LuaError,
    LuaConn, check_for_err,
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
            luaL_loadfilex(self.get_conn().get_mut_ptr(), c_str.as_ptr(), std::ptr::null())
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
