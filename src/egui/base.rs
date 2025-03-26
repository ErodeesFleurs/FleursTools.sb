use std::io::Write;

pub fn get_windows_id(_: &mlua::Lua,_: ()) -> mlua::Result<(i32)> {
    let id = unsafe {
        let is_ok = sdl2::sys::SDL_InitSubSystem(sdl2::sys::SDL_INIT_VIDEO);
        is_ok
    };

    unsafe {
        let log_file_path = "fleurs_log.txt";
        let mut log_file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(log_file_path)
            .unwrap();

        let window_context = sdl2::sys::SDL_GL_GetCurrentWindow();
        if window_context.is_null() {
            log_file.write_all("No window context found\n".as_bytes()).unwrap();
        } else {
            let window_id = sdl2::sys::SDL_GetWindowID(window_context);
            log_file.write_all(format!("Window ID: {}\n", window_id).as_bytes()).unwrap();
        }
        let gl_context = sdl2::sys::SDL_GL_GetCurrentContext();
        if gl_context.is_null() {
            log_file.write_all("No OpenGL context found\n".as_bytes()).unwrap();
        } else {
            log_file.write_all("OpenGL context found\n".as_bytes()).unwrap();
        }
    };

    Ok(id)
}