use glfw::{Context, SwapInterval, WindowEvent, WindowMode};
use crate::{get_glfw, input::{Action, get_input}};
use lead_mem::singleton_mut;
use lead_logger::{warn, critical};
use std::sync::mpsc::Receiver;
use toml::Value;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                           Update Cap                                           //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateCap {
    Vsync,
    Unlimited,
    Cap(u32),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Window                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

singleton_mut!(func: get_window, WINDOW, Window, Window::new());

pub struct Window {
    ptr: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    update_cap: UpdateCap,
    min_width: Option<u32>,
    min_height: Option<u32>,
    max_width: Option<u32>,
    max_height: Option<u32>,
    focused: bool,
    // Event Flags
    resized: bool,
    moved: bool,
}

impl Window {

    fn new() -> Window {
        let glfw = get_glfw();

        // Defaults Values
        let mut width = 800;
        let mut height = 600;
        let mut title = "Title".to_string();
        let mut update_cap = UpdateCap::Vsync;
        let mut min_width = None;
        let mut min_height = None;
        let mut max_width = None;
        let mut max_height = None;

        // Read file
        match std::fs::read_to_string("res/engine/window.toml") {
            Ok(str) => {
                match str.parse::<toml::Value>() {
                    Ok(parsed) => {
                        if let Some(window) = parsed.get("window") {
                            // Width
                            if let Some(Value::Integer(value)) = window.get("width") {
                                width = *value as u32;
                            } else {
                                warn!("Window", "Failed to read property 'width' in 'window' from config file");
                            }
                            // Height
                            if let Some(Value::Integer(value)) = window.get("height") {
                                height = *value as u32;
                            } else {
                                warn!("Window", "Failed to read property 'height' in 'window' from config file");
                            }
                            // Title
                            if let Some(Value::String(value)) = window.get("title") {
                                title = value.clone();
                            } else {
                                warn!("Window", "Failed to read property 'title' in 'window' from config file");
                            }
                            // Update cap
                            if let Some(value) = window.get("cap") {
                                match value {
                                    Value::String(str) if str.eq_ignore_ascii_case("vsync") => update_cap = UpdateCap::Vsync,
                                    Value::String(str) if str.eq_ignore_ascii_case("unlimited") => update_cap = UpdateCap::Unlimited,
                                    Value::Integer(value) if *value > 0 => update_cap = UpdateCap::Cap(*value as u32),
                                    _ => warn!("Window", "Failed to read property 'cap' in 'window' from config file"),
                                }
                            } else {
                                warn!("Window", "Failed to read property 'cap' in 'window' from config file");
                            }
                            // min-width
                            if let Some(value) = window.get("min-width") {
                                match value {
                                    Value::String(str) if str.eq_ignore_ascii_case("unset") => min_width = None,
                                    Value::Integer(value) if *value >= 0 => min_width = Some(*value as u32),
                                    _ => warn!("Window", "Failed to read property 'min-width' in 'window' from config file"),
                                }
                            } else {
                                warn!("Window", "Failed to read property 'min-width' in 'window' from config file");
                            }
                            // min-height
                            if let Some(value) = window.get("min-height") {
                                match value {
                                    Value::String(str) if str.eq_ignore_ascii_case("unset") => min_height = None,
                                    Value::Integer(value) if *value >= 0 => min_height = Some(*value as u32),
                                    _ => warn!("Window", "Failed to read property 'min-height' in 'window' from config file"),
                                }
                            } else {
                                warn!("Window", "Failed to read property 'min-height' in 'window' from config file");
                            }
                            // max-width
                            if let Some(value) = window.get("max-width") {
                                match value {
                                    Value::String(str) if str.eq_ignore_ascii_case("unset") => max_width = None,
                                    Value::Integer(value) if *value > 0 => max_width = Some(*value as u32),
                                    _ => warn!("Window", "Failed to read property 'max-width' in 'window' from config file"),
                                }
                            } else {
                                warn!("Window", "Failed to read property 'max-width' in 'window' from config file");
                            }
                            // max-height
                            if let Some(value) = window.get("max-height") {
                                match value {
                                    Value::String(str) if str.eq_ignore_ascii_case("unset") => max_height = None,
                                    Value::Integer(value) if *value > 0 => max_height = Some(*value as u32),
                                    _ => warn!("Window", "Failed to read property 'max-height' in 'window' from config file"),
                                }
                            } else {
                                warn!("Window", "Failed to read property 'max-height' in 'window' from config file");
                            }
                        } else {
                            warn!("Window", "Failed to read structure 'window' from config file");
                        }
                    }
                    Err(err) => warn!("Window", "Failed to parse config file: {}", err),
                }
            }
            Err(err) => warn!("Window", "Failed to read config file: {}", err),
        }

        if let Some((mut ptr, events)) = glfw.as_mut().create_window(width, height, title.as_str(), WindowMode::Windowed) {
            ptr.set_all_polling(true);
            ptr.make_current();

            ptr.set_size_limits(min_width, min_height, max_width, max_height);

            if update_cap == UpdateCap::Vsync {
                glfw.as_mut().set_swap_interval(SwapInterval::Sync(1));
            } else {
                glfw.as_mut().set_swap_interval(SwapInterval::None);
            }

            gl::load_with(|symbol| ptr.get_proc_address(symbol) as *const _);

            return Window {
                ptr,
                events,
                width,
                height,
                x: 0,
                y: 0,
                min_width,
                min_height,
                max_width,
                max_height,
                update_cap,
                focused: false,
                resized: false,
                moved: false,
            }
        } else {
            critical!("GLFW",  "Failed to create Window!");
        }
    }

    pub fn should_close(&self) -> bool {
        return self.ptr.should_close();
    }

    pub fn swap_buffers(&mut self) {
        self.ptr.swap_buffers();
    }

    pub fn process_events(&mut self) {
        // Reset flags
        self.resized = false;
        self.moved = false;

        let glfw = get_glfw();
        glfw.as_mut().poll_events();

        let input = get_input();
        input.as_mut().update();
        let mouse = &mut input.as_mut().mouse;
        let keyboard = &mut input.as_mut().keyboard;

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::Pos(x, y) => {
                    self.x = x;
                    self.y = y;
                    self.moved = true;
                }
                WindowEvent::Focus(focused) => self.focused = focused,
                WindowEvent::FramebufferSize(width, height) => {
                    self.width = width as u32;
                    self.height = height as u32;
                    self.resized = true;
                }
                WindowEvent::MouseButton(button, action, _) => {
                    match action {
                        glfw::Action::Press => mouse.buttons[button as usize] = Action::Pressed,
                        glfw::Action::Release => mouse.buttons[button as usize] = Action::Released,
                        _ => {}
                    }
                }
                WindowEvent::CursorPos(x, y) => {
                    mouse.x = x;
                    mouse.y = y;
                }
                WindowEvent::CursorEnter(cursor_in) => {
                    mouse.cursor_in = cursor_in;
                }
                WindowEvent::Scroll(x, y) => {
                    mouse.scroll_x = x;
                    mouse.scroll_y = y;
                }
                WindowEvent::Key(key, _, action, _) => {
                    match action {
                        glfw::Action::Press => keyboard.keys[key as usize] = Action::Pressed,
                        glfw::Action::Release => keyboard.keys[key as usize] = Action::Released,
                        _ => {}
                    }
                }
                WindowEvent::Char(chr) => keyboard.chars.push_front(chr),
                WindowEvent::FileDrop(files) => {
                    for file in files {
                        input.as_mut().dropped_files.push(file);
                    }
                }
                _ => {}
            }
        }
    }

    /* ====================================== Getters ======================================= */

    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }

    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }

    pub fn get_min_width(&self) -> Option<u32> {
        return self.min_width.clone();
    }

    pub fn get_min_height(&self) -> Option<u32> {
        return self.min_height.clone();
    }

    pub fn get_max_width(&self) -> Option<u32> {
        return self.max_width;
    }

    pub fn get_max_height(&self) -> Option<u32> {
        return self.max_height;
    }

    pub fn get_update_cap(&self) -> UpdateCap {
        return self.update_cap;
    }

    pub fn is_focused(&self) -> bool {
        return self.focused;
    }

    /* ====================================== Setters ======================================= */

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.ptr.set_size(width as i32, height as i32);
    }

    pub fn set_width(&mut self, width: u32) {
        self.set_size(width, self.height);
    }

    pub fn set_height(&mut self, height: u32) {
        self.set_size(self.width, height);
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
        self.ptr.set_pos(x, y);
    }

    pub fn set_x(&mut self, x: i32) {
        self.set_pos(x, self.y);
    }

    pub fn set_y(&mut self, y: i32) {
        self.set_pos(self.x, y);
    }

    pub fn set_update_cap(&mut self, update_cap: UpdateCap) {
        self.update_cap = update_cap;

        if update_cap == UpdateCap::Vsync {
            get_glfw().as_mut().set_swap_interval(SwapInterval::Sync(1));
        } else {
            get_glfw().as_mut().set_swap_interval(SwapInterval::None);
        }
    }

    pub fn set_size_limits(&mut self, min_width: Option<u32>, min_height: Option<u32>, max_width: Option<u32>, max_height: Option<u32>) {
        self.min_width = min_width;
        self.min_height = min_height;
        self.max_width = max_width;
        self.max_height = max_height;
        self.ptr.set_size_limits(self.min_width, self.min_height, self.max_width, self.max_height);
    }

    pub fn set_min_size(&mut self, min_width: Option<u32>, min_height: Option<u32>) {
        self.set_size_limits(min_width, min_height, self.max_width, self.max_height);
    }

    pub fn set_min_width(&mut self, min_width: Option<u32>) {
        self.set_min_size(min_width, self.min_height);
    }

    pub fn set_min_height(&mut self, min_height: Option<u32>) {
        self.set_min_size(self.min_width, min_height);
    }

    pub fn set_max_size(&mut self, max_width: Option<u32>, max_height: Option<u32>) {
        self.set_size_limits(self.min_width, self.min_height, max_width, max_height);
    }

    pub fn set_max_width(&mut self, max_width: Option<u32>) {
        self.set_max_size(max_width, self.max_height);
    }

    pub fn set_max_height(&mut self, max_height: Option<u32>) {
        self.set_max_size(self.max_width, max_height);
    }

    /* ======================================= Events ======================================= */

    pub fn resized(&self) -> bool {
        return self.resized;
    }

    pub fn moved(&self) -> bool {
        return self.moved;
    }

}