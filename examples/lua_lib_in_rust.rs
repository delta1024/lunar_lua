use lunar_lua::{
    ffi::{lua_CFunction, lua_State},
    LuaAuxLib, LuaConn, LuaCore, LuaStatePtr, LuaStateRef, State,
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
    n = rmath.add(3, 2)
    m = rmath.sub(n, 2)
    "#;
    if state.aux_load_buffer(&src, "example").is_err() || state.pcall(0, 0, 0).is_err() {
        let msg = state.to_string(-1);
        eprintln!("Error: {msg}");
    }
    state.get_global("m");
    println!("{}", state.to_number(-1));
}
