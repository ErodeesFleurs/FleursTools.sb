pub fn get_windows_id(_: &mlua::Lua,_: ()) -> mlua::Result<i32> {
    let id = unsafe {
        let is_ok = sdl2::sys::SDL_InitSubSystem(sdl2::sys::SDL_INIT_VIDEO);
        is_ok
    };

    Ok(id)
}