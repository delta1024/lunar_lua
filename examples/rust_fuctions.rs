use std::process::exit;

use lunar_lua::{ffi::lua_State, LuaAuxLib, LuaCore, LuaStatePtr, State};

extern "C" fn l_add_two(state: *mut lua_State) -> i32 {
    let state = LuaStatePtr::from(state);
    let conn = state.get_conn().borrow();
    let n = conn.aux_check_number(1);
    conn.push(n + 2.0);
    1
}

fn main() {
    let lua = State::new();
    lua.aux_open_libs();
    lua.push_c_function(Some(l_add_two));
    lua.set_global("addtwo");
    let src = "print(addtwo(3))";
    if lua.aux_load_buffer(src, "c_func").is_err() || lua.pcall(0, 0, 0).is_err() {
        let message = lua.to_string(-1);
        eprintln!("{}", message);
        exit(1);
    }
}
