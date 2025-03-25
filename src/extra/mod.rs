pub mod array;
pub mod functional;

pub fn register_structure(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let structure = lua.create_table()?;
    let array = lua.create_table()?;
    let array_constructor = lua.create_function(array::lua_generate)?;
    array.set("new", array_constructor)?;
    structure.set("array", array)?;
    Ok(structure)
}

pub fn register_function(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let functional = lua.create_table()?;
    let bind = lua.create_function(functional::bind)?;
    functional.set("bind", bind)?;
    let range = lua.create_function(functional::range)?;
    functional.set("range", range)?;
    let map = lua.create_function(functional::map)?;
    functional.set("map", map)?;
    let chain = lua.create_function(functional::chain)?;
    functional.set("chain", chain)?;
    Ok(functional)
}
