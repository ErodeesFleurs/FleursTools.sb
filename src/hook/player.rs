use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use mlua::FromLua;
use retour::static_detour;

use crate::hook::symbol::{PROGRAM_NAME, platform_base_addr, symbol_addr};
type IsAdminHookFn = extern "C" fn(player_ptr: u64) -> bool;
type SetBusyStateFn = extern "C" fn(player_ptr: u64, busy_state: PlayerBusyState);

static PLAYER_PTR: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Symbols {
    IsAdmin,
    SetBusyState,
}

#[cfg(target_os = "linux")]
const SYMBOL_ADDR_MAP: &[(Symbols, &str)] = &[
    (Symbols::IsAdmin, "Star::Player::isAdmin() const"),
    (
        Symbols::SetBusyState,
        "Star::Player::setBusyState(Star::PlayerBusyState)",
    ),
];
#[cfg(target_os = "windows")]
const SYMBOL_ADDR_MAP: &[(Symbols, &str)] = &[
    (Symbols::IsAdmin, "Star::Player::isAdmin"),
    (Symbols::SetBusyState, "Star::Player::setBusyState"),
];

static_detour! {
    static PlayerIsAdminHook: extern "C" fn(u64) -> bool;
}
static IS_ADMIN_HOOK_IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlayerBusyState {
    None,
    Chatting,
    Menu,
}

impl FromLua for PlayerBusyState {
    fn from_lua(lua_val: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        match lua_val {
            mlua::Value::String(s) => match s.to_string_lossy().as_ref() {
                "None" => Ok(PlayerBusyState::None),
                "Chatting" => Ok(PlayerBusyState::Chatting),
                "Menu" => Ok(PlayerBusyState::Menu),
                _ => Err(mlua::Error::FromLuaConversionError {
                    from: "string",
                    to: "PlayerBusyState".to_string(),
                    message: Some("Invalid PlayerBusyState value".to_string()),
                }),
            },
            _ => Err(mlua::Error::FromLuaConversionError {
                from: lua_val.type_name(),
                to: "PlayerBusyState".to_string(),
                message: Some("Expected a string".to_string()),
            }),
        }
    }
}

fn get_symbol_name(symbol: Symbols) -> anyhow::Result<&'static str> {
    SYMBOL_ADDR_MAP
        .iter()
        .find(|(sym, _)| *sym == symbol)
        .map(|(_, name)| *name)
        .ok_or_else(|| anyhow::anyhow!("Symbol '{:?}' not found in SYMBOL_ADDR_MAP", symbol))
}

fn get_main_ptr(_: &mlua::Lua, func: mlua::Function) -> anyhow::Result<u64> {
    let symbol_name = get_symbol_name(Symbols::IsAdmin)?;
    let base_addr = platform_base_addr(PROGRAM_NAME)?;
    let symbol_addr = match symbol_addr(symbol_name)? {
        Some(addr) => addr,
        None => {
            return Err(anyhow::anyhow!(
                "Symbol '{:?}' not found in program '{}'",
                Symbols::IsAdmin,
                PROGRAM_NAME
            ));
        }
    };

    let addr = base_addr + symbol_addr;

    unsafe {
        if IS_ADMIN_HOOK_IS_INITIALIZED.load(Ordering::Relaxed) == false {
            PlayerIsAdminHook.initialize(
                std::mem::transmute::<u64, IsAdminHookFn>(addr),
                player_is_admin_impl,
            )?;
            IS_ADMIN_HOOK_IS_INITIALIZED.store(true, Ordering::Relaxed);
        }
        PlayerIsAdminHook.enable()?;
    }

    func.call::<mlua::Value>(())?;

    unsafe { PlayerIsAdminHook.disable()? };
    return Ok(PLAYER_PTR.load(Ordering::Relaxed));
}

fn player_is_admin_impl(player_ptr: u64) -> bool {
    PLAYER_PTR.store(player_ptr, Ordering::Relaxed);
    PlayerIsAdminHook.call(player_ptr)
}

fn set_busy_state(player_ptr: u64, busy_state: PlayerBusyState) -> anyhow::Result<()> {
    let symbol_name = get_symbol_name(Symbols::SetBusyState)?;
    let base_addr = platform_base_addr(PROGRAM_NAME)?;
    let symbol_addr = match symbol_addr(symbol_name)? {
        Some(addr) => addr,
        None => {
            return Err(anyhow::anyhow!(
                "Symbol '{:?}' not found in program '{}'",
                Symbols::SetBusyState,
                PROGRAM_NAME
            ));
        }
    };
    let addr = base_addr + symbol_addr;
    unsafe {
        let func: SetBusyStateFn = std::mem::transmute::<u64, SetBusyStateFn>(addr);
        func(player_ptr, busy_state);
    }
    Ok(())
}

pub fn register_function(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let player = lua.create_table()?;

    let get_main_ptr = lua.create_function(|lua, func: mlua::Function| -> mlua::Result<u64> {
        let result = get_main_ptr(lua, func);
        result.map_err(|e| mlua::Error::RuntimeError(e.to_string()))
    })?;
    player.set("get_main_ptr", get_main_ptr)?;

    let set_busy_state = lua.create_function(
        |_, (player_ptr, busy_state): (u64, PlayerBusyState)| -> mlua::Result<()> {
            set_busy_state(player_ptr, busy_state)
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
        },
    )?;
    player.set("set_busy_state", set_busy_state)?;

    Ok(player)
}
