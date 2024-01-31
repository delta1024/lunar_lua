use lunar_lua::state::{State, LuaType};
fn stack_dump(lua: &mut State) {
let top = lua.get_top() as i32;

    for i in 1..=top {
        match lua.get_type(i) {
            LuaType::String => print!("{}", lua.to_string(i).unwrap_or_default()),
            LuaType::Boolean => print!("{}", lua.to_boolean(i)),
            LuaType::Number => print!("{}", lua.to_number(i)),
            _ => print!("{}", lua.type_name(i)),
        }

        print!(" ");
    }
    println!();

}
fn main() {
    let mut lua = State::new();
    lua.push(true);
    lua.push(10f64);
    lua.push(None);
    lua.push("hello");
    stack_dump(&mut lua);

    lua.push_value(-4);
    stack_dump(&mut lua);

    lua.replace(3);
    stack_dump(&mut lua);

    lua.set_top(6);
    stack_dump(&mut lua);

    lua.remove(-3);
    stack_dump(&mut lua);

    lua.set_top(-5);
    stack_dump(&mut lua);

}
