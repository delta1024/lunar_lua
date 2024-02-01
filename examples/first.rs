use lunar_lua::{LuaAuxLib, LuaCore, State};
fn main() {
    let lua = State::new();
    lua.aux_open_libs();
    let mut line = String::new();
    while std::io::stdin().read_line(&mut line).unwrap() != 0 {
        let error = lua.aux_load_buffer(&line, "line").is_err() || lua.pcall(0, 0, 0).is_err();
        if error {
            eprintln!("{}", lua.to_string(-1));
            lua.pop(1);
        }
        line.retain(|_| false);
    }
}
