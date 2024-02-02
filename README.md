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
