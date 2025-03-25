pub fn get_windows_id(_: &mlua::Lua,_: ()) -> mlua::Result<u32> {
    let id = unsafe {
        let raw_window = sdl2::sys::SDL_GL_GetCurrentWindow();
        sdl2::sys::SDL_GetWindowID(raw_window)
    };

    if id == 0 {
        let msg: std::borrow::Cow<'_, str> = unsafe {
            std::ffi::CStr::from_ptr(sdl2::sys::SDL_GetError()).to_string_lossy()
        };
        return Err(mlua::Error::RuntimeError(msg.to_string()));
    }

    Ok(id)
}