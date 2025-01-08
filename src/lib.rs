mod directives;
mod image;
mod pants;
mod template;

use mlua::prelude::*;

fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    let create_image = lua.create_function(|_, path: String| {
        let image = image::Image::new(&path);
        Ok(image)
    })?;
    // image_table.set("image", image)?;
    exports.set("image", create_image)?;

    exports.set(
        "generate_pants",
        lua.create_function(|_, (path, hide_body): (String, bool)| {
            let res = pants::generate_pants(path, hide_body);
            Ok(res)
        })?,
    )?;
    Ok(exports)
}

#[no_mangle]
pub unsafe extern "C-unwind" fn my_module(state: *mut mlua::lua_State) -> ::std::os::raw::c_int {
    mlua::Lua::entrypoint1(state, move |lua| lua_module(lua))
}
