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
//! fn main() {
//!     let lua = State(aux_new_state());
//!     lua.push(13f64);
//!     assert_eq!(13f64, lua.to_number(-1));
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
use ffi::lua_State;
/// lua auxilary library
pub use lua_aux::LuaAuxLib;
/// lua core library
pub use lua_core::LuaCore;
/// lua standard library
pub use lua_lib::LuaStandardLib;
pub use wrapper::*;

#[repr(transparent)]
pub struct LuaConnection<'state>(&'state lua_State);
impl LuaConnection<'_> {
    pub unsafe fn get_mut_ptr(&self) -> *mut lua_State {
        (self.0 as *const lua_State).cast_mut()
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
