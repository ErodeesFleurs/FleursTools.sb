use std::ffi;

use tklog::debug;

type PlayerIsAdminHookFn = extern "C" fn(addr: ffi::c_longlong) -> bool;

static mut PLAYER_PTR: ffi::c_longlong = 0;

static_detour!{
    static PlayerIsAdminHook: extern "C" fn(ffi::c_longlong) -> bool;
}

pub fn hook_player_is_admin(addr: u64) -> anyhow::Result<bool> {
    unsafe {
        PlayerIsAdminHook.initialize(
            std::mem::transmute::<u64, PlayerIsAdminHookFn>(addr),
            player_is_admin_impl,
        )?;
        PlayerIsAdminHook.enable()?;
    }

    Ok(true)
}

fn player_is_admin_impl(player_ptr: ffi::c_longlong) -> bool {
    unsafe {
        PLAYER_PTR = player_ptr;
        debug!("Player pointer set to: {}", PLAYER_PTR);
        PlayerIsAdminHook.call(player_ptr)
    }
}