pub mod back;
pub mod chest;
pub mod hat;
pub mod pants;

pub fn register(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let outfit = lua.create_table()?;

    let generate_pants = lua.create_function(pants::lua_generate)?;
    outfit.set("generate_pants", generate_pants)?;

    let generate_chest = lua.create_function(chest::lua_generate)?;
    outfit.set("generate_chest", generate_chest)?;

    let generate_back = lua.create_function(back::lua_generate)?;
    outfit.set("generate_back", generate_back)?;

    let generate_hat = lua.create_function(hat::lua_generate)?;
    outfit.set("generate_hat", generate_hat)?;

    Ok(outfit)
}
