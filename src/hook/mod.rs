mod egui;
mod player;
mod sdl;
mod symbol;

static PDB_PATH: &str = "./starbound.pdb";

pub fn register_function(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let hook = lua.create_table()?;

    hook.set("symbols", symbol::register_function(lua)?)?;
    hook.set("sdl", sdl::register_function(lua)?)?;
    hook.set("player", player::register_function(lua)?)?;

    Ok(hook)
}

pub fn init() -> anyhow::Result<()> {
    symbol::platform_parse_symbols(Some(PDB_PATH.to_string()))?;

    Ok(())
}
