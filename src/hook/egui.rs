use egui::{Key, RawInput};
use glow::HasContext;
use sdl2_sys::{SDL_BUTTON_LEFT, SDL_BUTTON_MIDDLE, SDL_BUTTON_RIGHT, SDL_Event, SDL_Scancode};

pub struct Platform {
    pub raw_input: RawInput,
}

impl Platform {
    pub fn handle_event(event: *mut SDL_Event) -> Option<Platform> {
        let mut raw_input = egui::RawInput {
            ..Default::default()
        };

        unsafe {
            match (*event).type_ {
                1025 /* SDL_MOUSEBUTTONDOWN */ | 1026 /* SDL_MOUSEBUTTONUP */ => {
                    let event = &(*event).button;
                    let pressed = (*event).type_ == 1025; // 判断是按下还是释放
                    if let Some(button) = match_mouse_button(event.button.into()) {
                        raw_input.events.push(egui::Event::PointerButton {
                            pos: egui::Pos2::new(event.x as f32, event.y as f32),
                            button,
                            pressed,
                            modifiers: Default::default(),
                        });
                    }
                }
                1024 /* SDL_MOUSEMOTION */ => {
                    let event = &(*event).motion;
                    raw_input.events.push(egui::Event::PointerMoved(egui::Pos2::new(
                        event.x as f32,
                        event.y as f32,
                    )));
                }
                768 /* SDL_KEYDOWN */ => {
                    let event = &(*event).key;
                    if let Some(key) = sdl_to_egui_key(event.keysym) {
                        raw_input.events.push(egui::Event::Key {
                            key,
                            pressed: true,
                            modifiers: Default::default(),
                            physical_key: Some(key), // Use the same key as the logical key
                            repeat: false, // Adjust based on your logic
                        });
                    }
                }
                769 /* SDL_KEYUP */ => {
                        let event = &(*event).key;
                        if let Some(key) = sdl_to_egui_key(event.keysym) {
                            raw_input.events.push(egui::Event::Key {
                                key,
                                pressed: false,
                                modifiers: Default::default(),
                                physical_key: Some(key), // Use the same key as the logical key
                                repeat: false, // Adjust based on your logic
                            });
                        }
                    }
                    _ => {}
                }
            };
            if !raw_input.events.is_empty() {
                Some(Platform { raw_input })
            } else {
                None
            }
        }

    }

fn match_mouse_button(button: u32) -> Option<egui::PointerButton> {
    match button {
        SDL_BUTTON_LEFT => Some(egui::PointerButton::Primary),
        SDL_BUTTON_MIDDLE => Some(egui::PointerButton::Middle),
        SDL_BUTTON_RIGHT => Some(egui::PointerButton::Secondary),
        _ => None,
    }
}

fn sdl_to_egui_key(key: sdl2_sys::SDL_Keysym) -> Option<egui::Key> {
    use SDL_Scancode::*;
    Some(match key.scancode {
        SDL_SCANCODE_LEFT => Key::ArrowLeft,
        SDL_SCANCODE_RIGHT => Key::ArrowRight,
        SDL_SCANCODE_UP => Key::ArrowUp,
        SDL_SCANCODE_DOWN => Key::ArrowDown,

        SDL_SCANCODE_ESCAPE => Key::Escape,
        SDL_SCANCODE_TAB => Key::Tab,
        SDL_SCANCODE_BACKSPACE => Key::Backspace,
        SDL_SCANCODE_SPACE => Key::Space,
        SDL_SCANCODE_RETURN => Key::Enter,

        SDL_SCANCODE_INSERT => Key::Insert,
        SDL_SCANCODE_HOME => Key::Home,
        SDL_SCANCODE_DELETE => Key::Delete,
        SDL_SCANCODE_END => Key::End,
        SDL_SCANCODE_PAGEDOWN => Key::PageDown,
        SDL_SCANCODE_PAGEUP => Key::PageUp,

        SDL_SCANCODE_KP_0 | SDL_SCANCODE_0 => Key::Num0,
        SDL_SCANCODE_KP_1 | SDL_SCANCODE_1 => Key::Num1,
        SDL_SCANCODE_KP_2 | SDL_SCANCODE_2 => Key::Num2,
        SDL_SCANCODE_KP_3 | SDL_SCANCODE_3 => Key::Num3,
        SDL_SCANCODE_KP_4 | SDL_SCANCODE_4 => Key::Num4,
        SDL_SCANCODE_KP_5 | SDL_SCANCODE_5 => Key::Num5,
        SDL_SCANCODE_KP_6 | SDL_SCANCODE_6 => Key::Num6,
        SDL_SCANCODE_KP_7 | SDL_SCANCODE_7 => Key::Num7,
        SDL_SCANCODE_KP_8 | SDL_SCANCODE_8 => Key::Num8,
        SDL_SCANCODE_KP_9 | SDL_SCANCODE_9 => Key::Num9,

        SDL_SCANCODE_A => Key::A,
        SDL_SCANCODE_B => Key::B,
        SDL_SCANCODE_C => Key::C,
        SDL_SCANCODE_D => Key::D,
        SDL_SCANCODE_E => Key::E,
        SDL_SCANCODE_F => Key::F,
        SDL_SCANCODE_G => Key::G,
        SDL_SCANCODE_H => Key::H,
        SDL_SCANCODE_I => Key::I,
        SDL_SCANCODE_J => Key::J,
        SDL_SCANCODE_K => Key::K,
        SDL_SCANCODE_L => Key::L,
        SDL_SCANCODE_M => Key::M,
        SDL_SCANCODE_N => Key::N,
        SDL_SCANCODE_O => Key::O,
        SDL_SCANCODE_P => Key::P,
        SDL_SCANCODE_Q => Key::Q,
        SDL_SCANCODE_R => Key::R,
        SDL_SCANCODE_S => Key::S,
        SDL_SCANCODE_T => Key::T,
        SDL_SCANCODE_U => Key::U,
        SDL_SCANCODE_V => Key::V,
        SDL_SCANCODE_W => Key::W,
        SDL_SCANCODE_X => Key::X,
        SDL_SCANCODE_Y => Key::Y,
        SDL_SCANCODE_Z => Key::Z,

        _ => return None,
    })
}

struct BlendState {
    mode_rgb: u32,
    mode_a: u32,
    mode_src_rgb: u32,
    mode_src_a: u32,
    mode_dst_rgb: u32,
    mode_dst_a: u32,
}

pub struct EguiGlow {
    pub egui_ctx: egui::Context,
    pub painter: egui_glow::Painter,

    sdl: Option<Platform>,
    shapes: Vec<egui::epaint::ClippedShape>,
    textures_delta: egui::TexturesDelta,
    blend_state: BlendState,
}

impl EguiGlow {
    pub fn new(gl: std::sync::Arc<glow::Context>) -> Self {
        let painter = egui_glow::Painter::new(gl, "", None, false).unwrap();

        let mode_rgb = unsafe { painter.gl().get_parameter_i32(glow::BLEND_EQUATION_RGB) as u32 };
        let mode_a = unsafe { painter.gl().get_parameter_i32(glow::BLEND_EQUATION_ALPHA) as u32 };
        let mode_src_rgb = unsafe { painter.gl().get_parameter_i32(glow::BLEND_SRC_RGB) as u32 };
        let mode_src_a = unsafe { painter.gl().get_parameter_i32(glow::BLEND_SRC_ALPHA) as u32 };
        let mode_dst_rgb = unsafe { painter.gl().get_parameter_i32(glow::BLEND_DST_RGB) as u32 };
        let mode_dst_a = unsafe { painter.gl().get_parameter_i32(glow::BLEND_DST_ALPHA) as u32 };

        Self {
            egui_ctx: Default::default(),
            painter,
            sdl: None,
            shapes: Default::default(),
            textures_delta: Default::default(),
            blend_state: BlendState {
                mode_rgb,
                mode_a,
                mode_src_rgb,
                mode_src_a,
                mode_dst_rgb,
                mode_dst_a,
            },
        }
    }

    pub fn run(&mut self, run_ui: impl FnMut(&egui::Context)) -> std::time::Duration {
        let input = match self.sdl.take() {
            Some(input) => input.raw_input,
            None => Default::default(),
        };
        // Get input here
        let egui::FullOutput {
            platform_output: _,
            textures_delta,
            shapes,
            pixels_per_point: _,
            viewport_output: _,
        } = self.egui_ctx.run(input, run_ui);

        self.shapes = shapes;
        self.textures_delta.append(textures_delta);
        std::time::Duration::from_secs(0)
    }

    pub fn paint(&mut self, dimensions: [u32; 2]) {
        let shapes = std::mem::take(&mut self.shapes);
        let mut textures_delta = std::mem::take(&mut self.textures_delta);

        for (id, image_delta) in textures_delta.set {
            self.painter.set_texture(id, &image_delta);
        }

        let clipped_primitives = self
            .egui_ctx
            .tessellate(shapes, self.egui_ctx.pixels_per_point());
        self.painter.paint_primitives(
            dimensions,
            self.egui_ctx.pixels_per_point(),
            &clipped_primitives,
        );

        for id in textures_delta.free.drain(..) {
            self.painter.free_texture(id);
        }

        // Restore the proper blend functions after rendering
        unsafe {
            self.painter.gl().blend_equation_separate(
                self.blend_state.mode_rgb as u32,
                self.blend_state.mode_a as u32,
            );
            self.painter.gl().blend_func_separate(
                self.blend_state.mode_src_rgb,
                self.blend_state.mode_dst_rgb,
                self.blend_state.mode_src_a,
                self.blend_state.mode_dst_a,
            );
        };
    }

    #[allow(unused)]
    pub fn destroy(&mut self) {
        self.painter.destroy()
    }

    pub fn set_raw_input(&mut self, input: Option<Platform>) {
        self.sdl = input;
    }
}
