use std::process::exit;

use lunar_lua::{LuaAuxLib, LuaCore, LuaStandardLib, LuaStateRef, State};
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

#[derive(Debug, Default)]
struct ColorTable {
    name: &'static str,
    red: f64,
    green: f64,
    blue: f64,
}
impl ColorTable {
    fn from_lua(state: LuaStateRef<'_>) -> Self {
        state.get_global("background");
        if !state.is_table(-1) {
            error!(state, "background is not a color table");
        }

        let red = get_field(state, "r");
        let green = get_field(state, "g");
        let blue = get_field(state, "b");
        Self {
            red,
            green,
            blue,
            ..Default::default()
        }
    }
    fn to_lua(&self, state: LuaStateRef<'_>) {
        state.new_table();
        set_field(state, "r", self.red);
        set_field(state, "g", self.green);
        set_field(state, "b", self.blue);
        state.set_global(&self.name);
    }
}
/// assumes table is on the stack top
fn get_field(state: LuaStateRef<'_>, key: &str) -> f64 {
    state.push_string(key);
    state.get_table(-2);
    if !state.is_number(-1) {
        error!(state, "invalid component in background color");
    }
    let result = state.to_number(-1);
    state.pop(1);
    result
}

/// assumes table is on the stack top
fn set_field(state: LuaStateRef<'_>, index: &str, value: f64) {
    state.push(index);
    state.push(value);
    state.set_table(-3);
}
fn load(file_name: &str) -> (ColorTable, i32, i32) {
    let lua = State::new();
    lua.open_base();
    lua.open_io();
    lua.open_string();
    lua.open_math();
    for color in COLOR_TABLE {
        color.to_lua(lua.borrow());
    }
    if lua.aux_load_file(file_name).is_err() || lua.pcall(0, 0, 0).is_err() {
        let message = lua.to_string(-1).to_string();
        let rf = lua.borrow();
        error!(rf, "cannot run configuration file: {}", message);
    }

    lua.get_global("width");
    lua.get_global("height");
    if !lua.is_number(-2) {
        let rf = lua.borrow();
        error!(rf, "`width' should be a number");
    }
    if !lua.is_number(-1) {
        let rf = lua.borrow();
        error!(rf, "`height' should be a number");
    }

    let (w, h) = (lua.to_number(-2) as i32, lua.to_number(-1) as i32);
    (
        ColorTable {
            name: "config",
            ..ColorTable::from_lua(lua.borrow())
        },
        w,
        h,
    )
}
const MAX_COLOR: f64 = 256f64;
const COLOR_TABLE: [ColorTable; 5] = [
    ColorTable {
        name: "WHITE",
        red: MAX_COLOR,
        green: MAX_COLOR,
        blue: MAX_COLOR,
    },
    ColorTable {
        name: "RED",
        red: MAX_COLOR,
        green: 0.0,
        blue: 0.0,
    },
    ColorTable {
        name: "GREEN",
        red: 0.0,
        green: MAX_COLOR,
        blue: 0.0,
    },
    ColorTable {
        name: "BLUE",
        red: 0.0,
        green: 0.0,
        blue: MAX_COLOR,
    },
    ColorTable {
        name: "BLACK",
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    },
];
fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let input = format!("{}/examples/table_manipulation/config.lua", dir);
    let (color, w, h) = load(&input);
    println!("color: {color:?}\nw: {w}, h: {h}");
}
