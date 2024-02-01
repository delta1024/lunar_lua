use std::process::exit;

use lunar_lua::{State, LuaStandardLib, LuaAuxLib, LuaCore};
macro_rules! error {
    ($self:ident, $($fmt:tt)*) => {
        lua_error($self, format!($($fmt)*))
    };
}
fn lua_error<T: ToString>(_: State, message: T) -> ! {
    eprintln!("{}", message.to_string());
    exit(1);
} 
fn load(file_name: &str) -> (i32, i32) {
    let lua = State::new();
    lua.open_base();
    lua.open_io();
    lua.open_string();
    lua.open_math();

    if lua.aux_load_file(file_name).is_err() || lua.pcall(0, 0, 0).is_err() {
        let message = lua.to_string(-1).to_string();
        error!(lua, "cannot run configuration file: {}", message );
    }

    lua.get_global("width");
    lua.get_global("height");
    if !lua.is_number(-2) {
        error!(lua, "`width' should be a number");
    }
    if !lua.is_number(-1) {
        error!(lua, "`height' should be a number");
    }

    (lua.to_number(-2)  as i32, lua.to_number(-1) as i32)

    
}
fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let input = format!("{}/examples/extend_your_appilication/config.lua", dir);
    let (w,h) = load(&input);
    println!("w: {w}, h: {h}"); 
}
