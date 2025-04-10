mod symbol;
mod state;

pub fn register_function(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let hook = lua.create_table()?;

    let init_symbols = lua.create_function(|lua: &mlua::Lua, path: Option<String>|  -> mlua::Result<mlua::Table> {
        let symbols = symbol::platform_parse_symbols(path).map_err(|e| mlua::Error::external(e))?;

        let symbols_table = lua.create_table()?;
        
        for (name, addr) in symbols {
            symbols_table.set(name, addr)?;
        }
        
        Ok(symbols_table)
    })?; 
    hook.set("init_symbols", init_symbols)?;

    let test_hook = lua.create_function(|_, (addr, name): (u64, String)| -> mlua::Result<bool> {
        let result = state::hook_func(addr, &name);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("test_hook", test_hook)?;

    let symbol_addr = lua.create_function(|_, name: String| -> mlua::Result<Option<u64>> {
        let addr = symbol::symbol_addr(&name);
        addr.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("symbol_addr", symbol_addr)?;
    
    Ok(hook)
}