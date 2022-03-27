use std::time::Instant;
use lead_mem::singleton;
use lead_mem::pointer::Pointer;
use glfw::{Glfw, InitError};
use lead_logger::*;
use crate::state::{get_state_manager, StateManager};
use crate::window::{get_window, UpdateCap};

pub mod window;
pub mod input;
pub mod state;

fn init_glfw() -> Glfw {
    match glfw::init(glfw::FAIL_ON_ERRORS) {
        Ok(glfw) => glfw,
        Err(err) => {
            error!("GLFW", "Failed to initialize glfw: {}", err);
            unreachable!()
        },
    }
}

singleton!(func: get_glfw, GLFW, Glfw, init_glfw());

pub fn start() {
    get_logger().as_mut().set_level(LogLevel::Debug);

    let window = get_window();
    let state_manager = get_state_manager();

    // Initialize
    state_manager.as_mut().init();

    // Loop
    let mut last = Instant::now();
    while !window.as_ref().should_close() {
        let delta = last.elapsed().as_secs_f64();
        let can_update = match window.as_ref().get_update_cap() {
            UpdateCap::Cap(cap) => delta >= 1.0 / cap as f64,
            _ => true
        };

        if can_update {
            // Window Update
            window.as_mut().process_events();

            // State update
            state_manager.as_mut().update(delta);

            // Swap Buffers
            window.as_mut().swap_buffers();
        }
    }

    // Dispose
    state_manager.as_mut().dispose();
}
