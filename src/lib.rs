//! # Safe bindings to lua.
//!
//! Like lua, lunar_lua aimes to be extensible.
//! For this reson all access to the lua api is provided
//! through three traits:
//! * [LuaCore]
//! * [LuaStandardLib]
//! * [LuaAuxLib]
//!
//! each trait exposses functions from thier respective c header.
//!
//! ```
//! use lunar_lua::{State, LuaCore};
//!
//! fn main() {
//!     let lua = State::new();
//!     lua.push(13f64);
//!     assert_eq!(13f64, lua.to_number(-1));
//! }
//! ```
//!
//! # Custom State
//! Creating a custom state is as easy as implemanting the [LuaConn] trait.
//!
//! ```
//! use lunar_lua::{lua_aux::aux_new_state, ffi::lua_State,LuaConn, LuaCore, LuaConnection};
//! struct State(*mut lua_State);
//! unsafe impl LuaConn for State {
//!  fn get_conn(&self) -> LuaConnection<'_> {
//!     unsafe {
//!     self.0.as_ref().expect("valid ptr expected").into()
//!     }
//!  }
//! }
//! impl Drop for State {
//!     fn drop(&mut self) {
//!         unsafe {
//!             self.get_conn().borrow().close_conn();
//!         }
//!     }
//! }
//! fn main() {
//!     let lua = State(aux_new_state());
//!     lua.push(13f64);
//!     assert_eq!(13f64, lua.to_number(-1));
//! }
//! ```
//! # Adding Rust Functions to Lua
//! ```
//! use std::process::exit;
//!
//! use lunar_lua::{ffi::lua_State, LuaStatePtr, LuaAuxLib, LuaCore, State};
//!
//! extern "C" fn l_add_two(state: *mut lua_State) -> i32 {
//!     let state = LuaStatePtr::from(state);
//!     let state = state.get_conn().borrow()
//!     let n = state.aux_check_number(1);
//!     state.push(n + 2.0);
//!     1
//! }
//!
//! fn main() {
//!     let lua = State::new();
//!     lua.aux_open_libs();
//!     lua.push_c_function(Some(l_add_two));
//!     lua.set_global("addtwo");
//!     let src = "result = addtwo(3)";
//!
//!     if lua.aux_load_buffer(src, "src").is_err() || lua.pcall(0,0,0).is_err() {
//!         let message = lua.to_string(-1);
//!         panic!("unable to compile expr: {}", message);
//!     }
//!
//!     lua.get_global("result");
//!     assert_eq!(5.0, lua.to_number(-1));
//! }
//! ```

/// Raw bindings to lua
#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
pub mod ffi;
pub mod lua_aux;
pub mod lua_core;
pub mod lua_lib;
/// Default lua wrapper.
pub mod wrapper;
use ffi::{lua_State, lua_close};
/// lua auxilary library
pub use lua_aux::LuaAuxLib;
/// lua core library
pub use lua_core::LuaCore;
/// lua standard library
pub use lua_lib::LuaStandardLib;
pub use wrapper::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct LuaConnection<'state>(&'state lua_State);
impl LuaConnection<'_> {
    pub unsafe fn get_mut_ptr(&self) -> *mut lua_State {
        (self.0 as *const lua_State).cast_mut()
    }
}
impl<'state> LuaConnection<'state> {
    pub unsafe fn from_raw(ptr: *mut lua_State) -> Self {
        Self(ptr.as_ref().unwrap())
    }
    pub fn borrow(self) -> LuaStateRef<'state> {
        self.into()
    }
}
impl<'state> From<&'state lua_State> for LuaConnection<'state> {
    fn from(value: &'state lua_State) -> Self {
        Self(value)
    }
}
pub unsafe trait LuaConn {
    fn get_conn(&self) -> LuaConnection<'_>;
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct LuaStateRef<'state>(LuaConnection<'state>);
impl LuaStateRef<'_> {
    /// Closes the underlying lua state.
    pub unsafe fn close_conn(self) {
        lua_close(self.get_conn().get_mut_ptr());
    }
}
unsafe impl<'state> LuaConn for LuaStateRef<'state> {
    fn get_conn(&self) -> LuaConnection<'_> {
        self.0
    }
}
impl<'state> From<LuaConnection<'state>> for LuaStateRef<'state> {
    fn from(value: LuaConnection<'state>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LuaStatePtr(*mut lua_State);
impl LuaStatePtr {
    pub fn get_conn(&self) -> LuaConnection<'_> {
        LuaConnection(unsafe { self.0.as_ref().unwrap() })
    }
}
impl From<*mut lua_State> for LuaStatePtr {
    fn from(value: *mut lua_State) -> Self {
        Self(value)
    }
}
