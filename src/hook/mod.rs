mod symbol;
mod state;

pub fn register_function(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let hook = lua.create_table()?;

    let symbols = lua.create_function(|lua: &mlua::Lua, path: Option<String>|  -> mlua::Result<mlua::Table> {
        let symbols = symbol::platform_parse_symbols(path);
        let table = lua.create_table()?;
        for (sym, addr) in symbols.map_err(|e| mlua::Error::external(e))? {
            table.set(sym, addr)?;
        }
        Ok(table)
    })?; 

    hook.set("symbols", symbols)?;

    let test_hook = lua.create_function(|_, (addr, name): (u64, String)| -> mlua::Result<bool> {
        let result = state::hook_func(addr, &name);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("test_hook", test_hook)?;

    #[cfg(target_os = "windows")]
    let open_pdb = lua.create_function(|_, path: String| -> mlua::Result<bool> {
        let result = symbol::open_pdb(&path);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    #[cfg(target_os = "windows")]
    hook.set("open_pdb", open_pdb)?;

    Ok(hook)
}