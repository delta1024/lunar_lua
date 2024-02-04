# Safe bindings to lua.

Like lua, lunar_lua aimes to be extensible.
For this reson all access to the lua api is provided
through three traits:
- [LuaCore](https://delta1024.github.io/lunar_lua/lunar_lua/lua_core/trait.LuaCore.html)
- [LuaStandardLib](https://delta1024.github.io/lunar_lua/lunar_lua/lua_lib/trait.LuaStandardLib.html)
- [LuaAuxLib](https://delta1024.github.io/lunar_lua/lunar_lua/lua_aux/trait.LuaAuxLib.html)
                                                                                         
each trait exposses functions from thier respective c header.
                                                                                         
```rust
use lunar_lua::{State, LuaCore};
                                                                                         
fn main() {
    let lua = State::new();
    lua.push(13f64);
    assert_eq!(13f64, lua.to_number(-1));
}
```
 # Custom State
 Creating a custom state is as easy as implemanting the [LuaConn](https://delta1024.github.io/lunar_lua/lunar_lua/trait.LuaConn.html) trait.
                                                                                          
 ```rust
 use lunar_lua::{lua_aux::aux_new_state, ffi::lua_State,LuaConn, LuaCore, LuaConnection};
 struct State(*mut lua_State);
 unsafe impl LuaConn for State {
  fn get_conn(&self) -> LuaConnection<'_> {
     unsafe {
     self.0.as_ref().expect("valid ptr expected").into()
     }
  }
 }
 impl Drop for State {
     fn drop(&mut self) {
         unsafe {
             self.get_conn().borrow().close_conn();
         }
     }
 }
 fn main() {
     let lua = State(aux_new_state());
     lua.push(13f64);
     assert_eq!(13f64, lua.to_number(-1));
 }
 ```
# Adding Rust Functions to Lua
```rust
use std::process::exit;
                                                                               
use lunar_lua::{ffi::lua_State, LuaStatePtr, LuaAuxLib, LuaCore, State};
                                                                               
extern "C" fn l_add_two(state: *mut lua_State) -> i32 {
    let state = LuaStatePtr::from(state);
    let n = state.aux_check_number(1);
    state.push(n + 2.0);
    1
}
                                                                               
fn main() {
    let lua = State::new();
    lua.aux_open_libs();
    lua.push_c_function(Some(l_add_two));
    lua.set_global("addtwo");
    let src = "result = addtwo(3)";
                                                                               
    if lua.aux_load_buffer(src, "src").is_err() || lua.pcall(0,0,0).is_err() {
        let message = lua.to_string(-1);
        panic!("unable to compile expr: {}", message);
    }
                                                                               
    lua.get_global("result");
    assert_eq!(5.0, lua.to_number(-1));
}
```
# Write a Lua Library in Rust
```rust

use lunar_lua::{
    ffi::{lua_CFunction, lua_State},
    LuaAuxLib, LuaCore, LuaStatePtr, LuaStateRef, State,
};

fn add(state: LuaStateRef<'_>) -> i32 {
    let a = state.aux_check_number(1);
    let b = state.aux_check_number(2);
    state.push(a + b);
    1
}
fn sub(state: LuaStateRef<'_>) -> i32 {
    let a = state.aux_check_number(1);
    let b = state.aux_check_number(2);
    state.push(a - b);
    1
}

extern "C" fn l_add(state: *mut lua_State) -> i32 {
    let state = LuaStatePtr::from(state);
    add(state.get_conn().borrow())
}
extern "C" fn l_sub(state: *mut lua_State) -> i32 {
    let state = LuaStatePtr::from(state);
    sub(state.get_conn().borrow())
}
const REGS: [(&'static str, lua_CFunction); 2] = [("add", Some(l_add)), ("sub", Some(l_sub))];
fn main() {
    let state = State::new();
    state.aux_new_lib(&REGS);
    state.set_global("rmath");
    let src = r#"
    local n = rmath.add(3, 2)
    m = rmath.sub(n, 2)
    "#;
    if state.aux_load_buffer(&src, "example").is_err() || state.pcall(0, 0, 0).is_err() {
        let msg = state.to_string(-1);
        eprintln!("Error: {msg}");
    }
    state.get_global("m");
    assert_eq!(3.0, state.to_number(-1));
}
```
