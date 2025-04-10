use std::ffi;
use std::sync::{Arc, LazyLock, Mutex};

use retour::static_detour;
use sdl2_sys::{SDL_Event, SDL_Window};
use tklog::debug;

use super::egui::{EguiGlow, Platform};

type GLSwapWindowFn = extern "C" fn(window: *mut SDL_Window);

type GLGetProcAddressFn = extern "C" fn(proc: *const ffi::c_char) -> *mut ffi::c_void;

type PollEventFn = extern "C" fn(event: *mut SDL_Event) -> ffi::c_int;

static EGUI_RENDERER: LazyLock<Mutex<Option<EguiGlow>>> = LazyLock::new(|| Mutex::new(None));

static_detour! {
    static GLSwapWindowHook: extern "C" fn(*mut SDL_Window);
    static PollEventHook: extern "C" fn(*mut SDL_Event) -> ffi::c_int;
}

pub fn hook_gl_swap_window(addr: u64) -> anyhow::Result<bool> {
    unsafe {
        GLSwapWindowHook
            .initialize(std::mem::transmute::<u64, GLSwapWindowFn>(addr), swap_window_impl)?;
        GLSwapWindowHook.enable()?;
    };

    Ok(true)
}

pub fn hook_poll_event(addr: u64) -> anyhow::Result<bool> {
    unsafe {
        PollEventHook.initialize(std::mem::transmute::<u64, PollEventFn>(addr), poll_event_impl)?;
        PollEventHook.enable()?;
    };

    Ok(true)
}

pub fn set_gl_get_proc_address(addr: u64) -> anyhow::Result<bool> {
    let addr = unsafe { std::mem::transmute::<u64, GLGetProcAddressFn>(addr) };

    let mut egui_renderer = EGUI_RENDERER
        .lock()
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    if let Some(_) = egui_renderer.as_mut() {
        return Ok(false);
    } else {
        let renderer = EguiGlow::new(
            Arc::new(unsafe {
                glow::Context::from_loader_function(|s| {
                    let c_str = std::ffi::CString::new(s).unwrap();
                    addr(c_str.as_ptr())
                })
            }),
            (1876, 992),
        );
        *egui_renderer = Some(renderer);
    }

    Ok(true)
}

pub fn swap_window_impl(window: *mut SDL_Window) {
    let mut egui_renderer = EGUI_RENDERER
        .lock()
        .unwrap();

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
        renderer.paint([1876, 992]);
    }


    GLSwapWindowHook.call(window);
}


pub fn poll_event_impl(event: *mut SDL_Event) -> ffi::c_int {
    let res = PollEventHook.call(event);
    if res != 0 {
        let platform = Platform::handle_event(event);

        let mut egui_renderer = EGUI_RENDERER
        .lock()
        .unwrap();

        if let Some(renderer) = egui_renderer.as_mut() {
            renderer.set_raw_input(platform);
        }
    }

    res
}
