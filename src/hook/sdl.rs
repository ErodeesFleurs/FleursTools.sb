use std::ffi;
use std::sync::{Arc, LazyLock, Mutex};

use retour::static_detour;
use sdl2_sys::{SDL_Event, SDL_Window};
use tklog::debug;

use super::egui::{EguiGlow, Platform};

type SDLGLSwapWindowFn = extern "C" fn(window: *mut SDL_Window);
type SDLGLGetProcAddressFn = extern "C" fn(proc: *const ffi::c_char) -> *mut ffi::c_void;
type SDLPollEventFn = extern "C" fn(event: *mut SDL_Event) -> ffi::c_int;
type SDLGetWindowSizeFn =
    extern "C" fn(window: *mut SDL_Window, w: *mut ffi::c_int, h: *mut ffi::c_int);

static EGUI_RENDERER: LazyLock<Mutex<Option<EguiGlow>>> = LazyLock::new(|| Mutex::new(None));

static SDL_GET_WINDOWS_SIZE_ADDR: Mutex<u64> = Mutex::new(0);

static_detour! {
    static GLSwapWindowHook: extern "C" fn(*mut SDL_Window);
    static PollEventHook: extern "C" fn(*mut SDL_Event) -> ffi::c_int;
}

fn hook_gl_swap_window(addr: u64) -> anyhow::Result<bool> {
    unsafe {
        GLSwapWindowHook.initialize(
            std::mem::transmute::<u64, SDLGLSwapWindowFn>(addr),
            swap_window_impl,
        )?;
        GLSwapWindowHook.enable()?;
    };

    Ok(true)
}

fn hook_poll_event(addr: u64) -> anyhow::Result<bool> {
    unsafe {
        PollEventHook.initialize(
            std::mem::transmute::<u64, SDLPollEventFn>(addr),
            poll_event_impl,
        )?;
        PollEventHook.enable()?;
    };

    Ok(true)
}

fn set_gl_get_proc_addr(addr: u64) -> anyhow::Result<bool> {
    let addr = unsafe { std::mem::transmute::<u64, SDLGLGetProcAddressFn>(addr) };

    let mut egui_renderer = EGUI_RENDERER
        .lock()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    if let Some(_) = egui_renderer.as_mut() {
        return Ok(false);
    } else {
        let renderer = EguiGlow::new(Arc::new(unsafe {
            glow::Context::from_loader_function(|s| {
                let c_str = std::ffi::CString::new(s).unwrap();
                addr(c_str.as_ptr())
            })
        }));
        *egui_renderer = Some(renderer);
    }

    Ok(true)
}

fn set_get_window_size_addr(addr: u64) -> anyhow::Result<bool> {
    let mut inner: std::sync::MutexGuard<'_, u64> = SDL_GET_WINDOWS_SIZE_ADDR
        .lock()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    *inner = addr;

    Ok(true)
}

fn swap_window_impl(window: *mut SDL_Window) {
    let mut egui_renderer = EGUI_RENDERER.lock().unwrap();

    if let Some(renderer) = egui_renderer.as_mut() {
        renderer.run(|egui_ctx| {
            egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                ui.heading("Hello World!");
                if ui.button("Log").clicked() {
                    debug!("Button Clicked");
                }
            });
            egui::Window::new("Atcha").show(egui_ctx, |ui| ui.label("egui + glow + sdl"));
        });

        let window_size = unsafe {
            let mut width: ffi::c_int = 0;
            let mut height: ffi::c_int = 0;

            let addr = SDL_GET_WINDOWS_SIZE_ADDR
                .lock()
                .map_err(|e| anyhow::anyhow!(e.to_string()))
                .unwrap();

            if *addr != 0 {
                let get_window_size: SDLGetWindowSizeFn =
                    std::mem::transmute::<u64, SDLGetWindowSizeFn>(*addr);
                get_window_size(window, &mut width, &mut height);
            }

            [width as u32, height as u32]
        };

        renderer.paint(window_size);
    }

    GLSwapWindowHook.call(window);
}

fn poll_event_impl(event: *mut SDL_Event) -> ffi::c_int {
    let res = PollEventHook.call(event);
    if res != 0 {
        let platform = Platform::handle_event(event);

        let mut egui_renderer = EGUI_RENDERER.lock().unwrap();

        if let Some(renderer) = egui_renderer.as_mut() {
            renderer.set_raw_input(platform);
        }
    }

    res
}

pub fn register_function(lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let sdl = lua.create_table()?;

    let hook_gl_swap_window = lua.create_function(|_, addr: u64| -> mlua::Result<bool> {
        let result = hook_gl_swap_window(addr);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    sdl.set("hook_gl_swap_window", hook_gl_swap_window)?;

    let hook_poll_event = lua.create_function(|_, addr: u64| -> mlua::Result<bool> {
        let result = hook_poll_event(addr);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    sdl.set("hook_poll_event", hook_poll_event)?;

    let set_gl_get_proc_addr = lua.create_function(|_, addr: u64| -> mlua::Result<bool> {
        let result = set_gl_get_proc_addr(addr);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    sdl.set("set_gl_get_proc_addr", set_gl_get_proc_addr)?;

    let set_get_window_size_addr = lua.create_function(|_, addr: u64| -> mlua::Result<bool> {
        let result = set_get_window_size_addr(addr);
        result.map_err(|e| mlua::Error::external(e))
    })?;
    sdl.set("set_get_window_size_addr", set_get_window_size_addr)?;

    Ok(sdl)
}
