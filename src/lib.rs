mod extra;
mod outfit;
mod utils;

use mlua::prelude::*;

fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    let outfit = outfit::register(lua)?;
    exports.set("outfit", outfit)?;

    let structure = extra::register_structure(lua)?;
    exports.set("structure", structure)?;

    let functional = extra::register_functional(lua)?;
    exports.set("functional", functional)?;
    Ok(exports)
}

#[no_mangle]
pub unsafe extern "C" fn fleurs_module(state: *mut mlua::lua_State) -> ::std::os::raw::c_int {
    mlua::Lua::entrypoint1(state, move |lua| lua_module(lua))
}
