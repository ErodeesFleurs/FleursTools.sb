mod directives;
mod image;
mod pants;
mod template;

use mlua::prelude::*;

fn hello(_: &Lua, name: String) -> LuaResult<()> {
    println!("hello, {}!", name);
    Ok(())
}

fn t_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("hello", lua.create_function(hello)?)?;
    // let image_table = lua.create_table()?;

    let create_image = lua.create_function(|_, path: String| {
        let image = image::Image::new(&path);
        Ok(image)
    })?;
    // image_table.set("image", image)?;
    exports.set("image", create_image)?;

    exports.set(
        "generate_pants",
        lua.create_function(|_, path: String| {
            let res = pants::generate_pants(path);
            Ok(res)
        })?,
    )?;
    Ok(exports)
}

#[no_mangle]
pub unsafe extern "C-unwind" fn my_module(state: *mut mlua::lua_State) -> ::std::os::raw::c_int {
    mlua::Lua::entrypoint1(state, move |lua| t_module(lua))
}
