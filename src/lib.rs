mod asset;
mod extra;
mod directives;
mod utils;
mod stream;

use mlua::prelude::*;

fn lua_module(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    let directive = directives::register_function(lua)?;
    exports.set("directive", directive)?;

    let structure = extra::register_structure(lua)?;
    exports.set("structure", structure)?;

    let functional = extra::register_function(lua)?;
    exports.set("functional", functional)?;
    
    Ok(exports)
}

#[unsafe(no_mangle)]
pub extern "C" fn fleurs_module(state: *mut mlua::lua_State) -> ::std::os::raw::c_int {
    unsafe { mlua::Lua::entrypoint1(state, move |lua| lua_module(lua)) }
}
