mod symbol;
mod state;
mod egui;
mod sdl;
mod call;

static PDB_PATH: &str = "./starbound.pdb";

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
    
    let base_addr = lua.create_function(|_, name: String| -> mlua::Result<u64> {
        let addr = symbol::platform_base_addr(&name);
        addr.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("base_addr", base_addr)?; 

    let symbol_addr = lua.create_function(|_, name: String| -> mlua::Result<Option<u64>> {
        let addr = symbol::symbol_addr(&name);
        addr.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("symbol_addr", symbol_addr)?;

    let dynamic_symbol_addr = lua.create_function(|_, (module, name): (String, String)| -> mlua::Result<u64> {
        let addr = symbol::platform_dynamic_symbol_addr(&module, &name);
        addr.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("dynamic_symbol_addr", dynamic_symbol_addr)?;
    
    let sdl_gl_swap_window_hook = lua.create_function(|_, addr: u64| -> mlua::Result<bool> {
        let result = sdl::hook_gl_swap_window(addr);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("sdl_gl_swap_window_hook", sdl_gl_swap_window_hook)?;

    let sdl_poll_event_hook = lua.create_function(|_, addr: u64| -> mlua::Result<bool> {
        let result = sdl::hook_poll_event(addr);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("sdl_poll_event_hook", sdl_poll_event_hook)?;    

    let set_sdl_gl_get_proc_addr = lua.create_function(|_, addr: u64| -> mlua::Result<bool> {
        let result = sdl::set_gl_get_proc_addr(addr);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("set_sdl_gl_get_proc_addr", set_sdl_gl_get_proc_addr)?;

    let set_sdl_get_window_size_addr = lua.create_function(|_, addr: u64| -> mlua::Result<bool> {
        let result = sdl::set_get_window_size_addr(addr);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    hook.set("set_sdl_get_window_size_addr", set_sdl_get_window_size_addr)?;

    Ok(hook)
}

pub fn init() -> anyhow::Result<()> {
    symbol::platform_parse_symbols(Some(PDB_PATH.to_string()))?;

    Ok(())
}