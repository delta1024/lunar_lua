use std::process::exit;

use lunar_lua::{LuaAuxLib, LuaCore, LuaStandardLib, LuaStateRef, State};
const LUA_SRC: &'static str = r#"
function f (x, y)
  return (x^2 * math.sin(y))/(1 - x)
end
"#;
macro_rules! error {
    ($self:ident, $($fmt:tt)*) => {
        lua_error($self, format!($($fmt)*))
    };
}
fn lua_error<T: ToString>(c: LuaStateRef<'_>, message: T) -> ! {
    eprintln!("{}", message.to_string());
    unsafe {
        c.close_conn();
    }
    exit(1);
}
fn call_f(state: LuaStateRef<'_>, x: f64, y: f64) -> f64 {
    state.get_global("f");
    state.push(x);
    state.push(y);

    if state.pcall(2, 1, 0).is_err() {
        error!(state, "error running function `f': {}", state.to_string(-1));
    }

    if !state.is_number(-1) {
        error!(state, "function `f' must return a number");
    }
    let z = state.to_number(-1);
    state.pop(1);
    z
}
fn main() {
    let lua = State::new();
    lua.open_math();
    if lua.aux_load_buffer(LUA_SRC, "example").is_err() || lua.pcall(0, 0, 0).is_err() {
        let message = lua.to_string(-1).to_string();
        let rf = lua.borrow();
        error!(rf, "cannot run configuration file: {}", message);
    }
    println!("f(6,3)={}", call_f(lua.borrow(), 6.0, 3.0));
}
