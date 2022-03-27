pub use glfw::{Key, MouseButton, MouseButtonLeft, MouseButtonRight, MouseButtonMiddle};
use std::collections::VecDeque;
use lead_mem::singleton_mut;
use std::path::PathBuf;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Input                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

singleton_mut!(func: get_input, INPUT, Input, Input::new());

pub struct Input {
    pub(super) mouse: Mouse,
    pub(super) keyboard: Keyboard,
    pub(super) dropped_files: Vec<PathBuf>,
}

impl Input {

    fn new() -> Input {
        return Input {
            mouse: Mouse::new(),
            keyboard: Keyboard::new(),
            dropped_files: Vec::new()
        }
    }

    pub(super) fn update(&mut self) {
        self.dropped_files.clear();
        self.mouse.update();
        self.keyboard.update();
    }

    /* ====================================== Getters ======================================= */

    pub fn mouse(&self) -> &Mouse {
        return &self.mouse;
    }

    pub fn keyboard(&self) -> &Keyboard {
        return &self.keyboard;
    }

    pub fn get_dropped_files(&self) -> Vec<PathBuf> {
        return self.dropped_files.clone();
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Action                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Action {
    Pressed,
    Down,
    Released,
    Up
}

impl Action {

    pub fn is_pressed(&self) -> bool {
        matches!(self, Action::Pressed)
    }

    pub fn is_down(&self) -> bool {
        matches!(self, Action::Down)
    }

    pub fn is_released(&self) -> bool {
        matches!(self, Action::Released)
    }

    pub fn is_up(&self) -> bool {
        matches!(self, Action::Up)
    }

    pub fn is_pressed_or_down(&self) -> bool {
        matches!(self, Action::Pressed | Action::Down)
    }

    pub fn is_pressed_or_released(&self) -> bool {
        matches!(self, Action::Pressed | Action::Released)
    }

    pub fn is_pressed_or_up(&self) -> bool {
        matches!(self, Action::Pressed | Action::Up)
    }

    pub fn is_down_or_released(&self) -> bool {
        matches!(self, Action::Down | Action::Released)
    }

    pub fn is_down_or_up(&self) -> bool {
        matches!(self, Action::Down | Action::Up)
    }

    pub fn is_released_or_up(&self) -> bool {
        matches!(self, Action::Released | Action::Up)
    }

}

impl Default for Action {

    fn default() -> Self {
        return Action::Up;
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Mouse                                              //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Mouse {
    pub (super) x: f64,
    pub (super) y: f64,
    pub (super) last_x: f64,
    pub (super) last_y: f64,
    pub (super) cursor_in: bool,
    pub (super) scroll_x: f64,
    pub (super) scroll_y: f64,
    pub (super) buttons: [Action; (glfw::ffi::MOUSE_BUTTON_LAST + 1) as usize],
}

impl Mouse {

    fn new() -> Mouse {
        return Mouse {
            x: 0.0,
            y: 0.0,
            last_x: 0.0,
            last_y: 0.0,
            cursor_in: false,
            scroll_x: 0.0,
            scroll_y: 0.0,
            buttons: [Action::default(); (glfw::ffi::MOUSE_BUTTON_LAST + 1) as usize]
        }
    }

    fn update(&mut self) {
        self.last_x = self.x;
        self.last_y = self.y;
        self.scroll_x = 0.0;
        self.scroll_y = 0.0;

        for button in self.buttons.iter_mut() {
            match button {
                Action::Pressed => *button = Action::Down,
                Action::Released => *button = Action::Up,
                _ => {}
            }
        }
    }

    /* ====================================== Getters ======================================= */

    pub fn get_x(&self) -> f64 {
        return self.x;
    }

    pub fn get_y(&self) -> f64 {
        return self.y;
    }

    pub fn get_last_x(&self) -> f64 {
        return self.last_x;
    }

    pub fn get_last_y(&self) -> f64 {
        return self.last_y;
    }

    pub fn get_dx(&self) -> f64 {
        return self.x - self.last_x;
    }

    pub fn get_dy(&self) -> f64 {
        return self.y - self.last_y;
    }

    pub fn is_cursor_in(&self) -> bool {
        return self.cursor_in;
    }

    pub fn get_scroll_x(&self) -> f64 {
        return self.scroll_x;
    }

    pub fn is_scroll_left(&self) -> bool {
        return self.scroll_x > 0.0;
    }

    pub fn is_scroll_right(&self) -> bool {
        return self.scroll_x < 0.0;
    }

    pub fn is_scroll_x(&self) -> bool {
        return self.scroll_x != 0.0;
    }

    pub fn get_scroll_y(&self) -> f64 {
        return self.scroll_y;
    }

    pub fn is_scroll_up(&self) -> bool {
        return self.scroll_y > 0.0;
    }

    pub fn is_scroll_down(&self) -> bool {
        return self.scroll_y < 0.0;
    }

    pub fn is_scroll_y(&self) -> bool {
        return self.scroll_y != 0.0;
    }

    pub fn get_button(&self, button: MouseButton) -> Action {
        return self.buttons[button as usize];
    }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                            Keyboard                                            //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Keyboard {
    pub (super) keys: [Action; (glfw::ffi::KEY_LAST + 1) as usize],
    pub (super) chars: VecDeque<char>,
}

impl Keyboard {

    pub fn new() -> Keyboard {
        return Keyboard {
            keys: [Action::default(); (glfw::ffi::KEY_LAST + 1) as usize],
            chars: VecDeque::new()
        }
    }

    pub(super) fn update(&mut self) {
        self.chars.clear();
        for key in self.keys.iter_mut() {
            match key {
                Action::Pressed => *key = Action::Down,
                Action::Released => *key = Action::Up,
                _ => {}
            }
        }
    }

    /* ====================================== Getters ======================================= */

    pub fn get_key(&self, key: Key) -> Action {
        return self.keys[key as usize];
    }

    pub fn get_chars(&self) -> VecDeque<char> {
        return self.chars.clone();
    }

}