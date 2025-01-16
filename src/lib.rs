mod back;
mod chest;
mod directives;
mod hat;
mod image;
mod pants;
mod template;

use mlua::prelude::*;

fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set(
        "generate_pants",
        lua.create_function(|_, (path, hide_body): (String, bool)| {
            let res = pants::generate_pants(path, hide_body);
            Ok(res)
        })?,
    )?;

    exports.set(
        "generate_chest",
        lua.create_function(
            |_, (chest_path, front_sleeve_path, back_sleeve_path): (String, String, String)| {
                let res = chest::generate_chest(chest_path, front_sleeve_path, back_sleeve_path);
                Ok(res)
            },
        )?,
    )?;

    exports.set(
        "generate_back",
        lua.create_function(|_, back_path: String| {
            let res = back::generate_back(back_path);
            Ok(res)
        })?,
    )?;

    exports.set(
        "generate_hat",
        lua.create_function(|_, hat_path: String| {
            let res = hat::generate_hat(hat_path);
            Ok(res)
        })?,
    )?;

    Ok(exports)
}

#[no_mangle]
pub unsafe extern "C" fn fleurs_module(state: *mut mlua::lua_State) -> ::std::os::raw::c_int {
    mlua::Lua::entrypoint1(state, move |lua| lua_module(lua))
}
