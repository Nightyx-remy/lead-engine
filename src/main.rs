use lead_core::input::get_input;
use lead_core::start;
use lead_core::state::{get_state_manager, IState};
use lead_logger::*;

pub struct MainState;

impl IState for MainState {
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
            println!("Scroll");
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
