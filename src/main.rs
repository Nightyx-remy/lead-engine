use lead_core::state::{get_state_manager, TState};
use lead_core::input::get_input;
use lead_core::start;

pub struct MainState;

impl TState for MainState {
    fn init(&mut self) {
        println!("Initialized");
    }

    fn open(&mut self) {
        println!("Opened");
    }

    fn update(&mut self, delta: f64) {
        let input = get_input();
        let mouse = input.as_mut().mouse();

        if mouse.is_scroll_y() {
            println!("FPS: {}", 1.0 / delta);
        }
    }

    fn close(&mut self) {
        println!("Closed");
    }

    fn dispose(&mut self) {
        println!("Disposed");
    }
}

fn main() {
    let state_manager = get_state_manager();
    state_manager.as_mut().register(MainState);
    state_manager.as_mut().open::<MainState>();

    start();
}
