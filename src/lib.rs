mod outfit;
mod utils;

use crate::utils::image::read_image;
use mlua::prelude::*;

fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set(
        "generate_pants",
        lua.create_function(|_, (userdata, hide_body): (LuaAnyUserData, bool)| {
            let res = outfit::pants::generate_pants(*read_image(userdata), hide_body);
            Ok(res)
        })?,
    )?;

    exports.set(
        "generate_chest",
        lua.create_function(
            |_,
             (torso_userdata, front_sleeve_userdata, back_sleeve_userdata): (
                LuaAnyUserData,
                LuaAnyUserData,
                LuaAnyUserData,
            )| {
                let res = outfit::chest::generate_chest(
                    *read_image(torso_userdata),
                    *read_image(front_sleeve_userdata),
                    *read_image(back_sleeve_userdata),
                );
                Ok(res)
            },
        )?,
    )?;

    exports.set(
        "generate_back",
        lua.create_function(|_, userdata: LuaAnyUserData| {
            let res = outfit::back::generate_back(*read_image(userdata));
            Ok(res)
        })?,
    )?;

    exports.set(
        "generate_hat",
        lua.create_function(|_, userdata: LuaAnyUserData| {
            let res = outfit::hat::generate_hat(*read_image(userdata));
            Ok(res)
        })?,
    )?;

    Ok(exports)
}

#[no_mangle]
pub unsafe extern "C" fn fleurs_module(state: *mut mlua::lua_State) -> ::std::os::raw::c_int {
    mlua::Lua::entrypoint1(state, move |lua| lua_module(lua))
}
