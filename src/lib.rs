/// Raw bindings to lua
#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
pub mod ffi;
pub mod lua_aux;
pub mod lua_core;
pub mod lua_lib;
pub mod wrapper;
use ffi::lua_State;
pub use lua_aux::LuaAuxLib;
pub use lua_core::LuaCore;
pub use lua_lib::LuaStandardLib;
pub use wrapper::*;

#[repr(transparent)]
pub struct LuaConnection<'state>(&'state lua_State);
impl LuaConnection<'_> {
    unsafe fn get_mut_ptr(&self) -> *mut lua_State {
        (self.0 as *const lua_State).cast_mut()
    }
}
pub unsafe trait LuaConn {
    fn get_conn(&self) -> LuaConnection<'_>;
}
