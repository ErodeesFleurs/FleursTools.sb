mod symbol;

pub fn register_function(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let hook = lua.create_table()?;

    let symbols = lua.create_function(|lua: &mlua::Lua, path: String|  -> mlua::Result<mlua::Table> {
        let symbols = symbol::platform_parse_symbols(&path);
        let table = lua.create_table()?;
        for (sym, addr) in symbols.map_err(|e| mlua::Error::external(e))? {
            table.set(sym, addr)?;
        }
        Ok(table)
    })?; 

    hook.set("symbols", symbols)?;

    Ok(hook)
}