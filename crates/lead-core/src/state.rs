use lead_mem::{ObjectState, singleton_mut, pointer::MutPointer};
use std::any::{Any, TypeId};
use lead_logger::warn;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             TState                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait TState: Any {

    fn init(&mut self);
    fn open(&mut self);
    fn update(&mut self, delta: f64);
    fn close(&mut self);
    fn dispose(&mut self);

}

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                         State Manager                                          //
////////////////////////////////////////////////////////////////////////////////////////////////////

singleton_mut!(func: get_state_manager, STATE_MANAGER, StateManager, StateManager::new());

pub struct StateManager {
    states: Vec<Box<dyn TState>>,
    current: Option<MutPointer<Box<dyn TState>>>,
    object_state: ObjectState,
}

impl StateManager {

    fn new() -> StateManager {
        return StateManager {
            states: Vec::new(),
            current: None,
            object_state: ObjectState::Created,
        }
    }

    pub fn init(&mut self) {
        match self.object_state {
            ObjectState::Created => {
                for state in self.states.iter_mut() {
                    state.init();
                }

                if let Some(current) = &self.current {
                    current.as_mut().open();
                }

                self.object_state = ObjectState::Initialized;
            }
            ObjectState::Initialized => warn!("StateManager", "Failed to initialize StateManager, already initialized"),
            ObjectState::Disposed => warn!("StateManager", "Failed to initialize StateManager, already disposed"),
        }
    }

    pub fn register<State: TState + 'static>(&mut self, mut state: State) {
        match self.object_state {
            ObjectState::Disposed => warn!("StateManager", "Failed to register state '{}', already disposed", std::any::type_name::<State>()),
            ObjectState::Initialized => state.init(),
            _ => {}
        }

        self.states.push(Box::new(state));
    }

    pub fn open<State: TState + 'static>(&mut self) {
        if self.object_state == ObjectState::Disposed {
           warn!("StateManager", "Failed to open state '{}', already disposed", std::any::type_name::<State>());
           return;
        }

        // Close current state
        self.close();

        // Find matching state
        for state in self.states.iter_mut() {
            if (**state).type_id() == TypeId::of::<State>() {
                if self.object_state == ObjectState::Initialized {
                    state.open();
                }
                self.current = Some(MutPointer::new(state));
                return;
            }
        }

        // No matching state
        warn!("StateManager", "Failed to open state '{}', not found", std::any::type_name::<State>());
    }

    pub fn update(&mut self, delta: f64) {
        if self.object_state == ObjectState::Initialized {
            if let Some(current) = &self.current {
                current.as_mut().update(delta);
            }
        }
    }

    pub fn close(&mut self) {
        if let Some(state) = self.current.take() {
            if self.object_state == ObjectState::Initialized {
                state.as_mut().close();
            }
        }
    }

    pub fn dispose(&mut self) {
        match self.object_state {
            ObjectState::Created => warn!("StateManager", "Failed to dispose StateManager, not initialized"),
            ObjectState::Initialized => {
                if let Some(current) = self.current.take() {
                    current.as_mut().close();
                }

                for state in self.states.iter_mut() {
                    state.dispose();
                }

                self.object_state = ObjectState::Disposed;
            }
            ObjectState::Disposed => warn!("StateManager", "Failed to dispose StateManager, already disposed"),
        }
    }

}