# Safe bindings to lua.                                                                  ``

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
