mod outfit;
mod utils;

use mlua::prelude::*;

fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let outfit = lua.create_table()?;
    outfit.set(
        "generate_pants",
        lua.create_function(outfit::pants::lua_generate)?,
    )?;

    outfit.set(
        "generate_chest",
        lua.create_function(outfit::chest::lua_generate)?,
    )?;

    outfit.set(
        "generate_back",
        lua.create_function(outfit::back::lua_generate)?,
    )?;

    outfit.set(
        "generate_hat",
        lua.create_function(outfit::hat::lua_generate)?,
    )?;
    exports.set("outfit", outfit)?;

    Ok(exports)
}

#[no_mangle]
pub unsafe extern "C" fn fleurs_module(state: *mut mlua::lua_State) -> ::std::os::raw::c_int {
    mlua::Lua::entrypoint1(state, move |lua| lua_module(lua))
}
