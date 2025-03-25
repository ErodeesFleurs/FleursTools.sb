mod base;

pub fn register_function(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let functional = lua.create_table()?;
    let window_id = lua.create_function(base::get_windows_id)?;
    functional.set("get_windows_id", window_id)?;
    Ok(functional)
}