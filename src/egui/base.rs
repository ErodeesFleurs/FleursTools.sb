pub fn get_windows_id(_: &mlua::Lua,_: ()) -> mlua::Result<i32> {
    let id = unsafe {
        let is_ok = sdl2::sys::SDL_InitSubSystem(sdl2::sys::SDL_INIT_VIDEO);
        is_ok
    };

    unsafe {
        let window_context = sdl2::sys::SDL_GL_GetCurrentWindow();
        if window_context.is_null() {
            println!("No window context found");
        } else {
            let window_id = sdl2::sys::SDL_GetWindowID(window_context);
            println!("Window ID: {}", window_id);
        }
        let gl_context = sdl2::sys::SDL_GL_GetCurrentContext();
        if gl_context.is_null() {
            println!("No OpenGL context found");
        } else {
            println!("OpenGL context found");
        }
    };

    Ok(id)
}