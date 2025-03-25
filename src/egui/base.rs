pub fn get_windows_id(_: &mlua::Lua,_: ()) -> mlua::Result<u32> {
    let id = unsafe {
        let raw_window = sdl2::sys::SDL_GL_GetCurrentWindow();
        sdl2::sys::SDL_GetWindowID(raw_window)
    };

    Ok(id)
}