use super::actions::{Action, TimedAction};
use super::home::HomeState;
use super::type_test_new::TypeTestState;
use crate::keyboard::Keyboard;

pub enum StateError {
    NoActionYet,
}

pub enum View {
    Home,
    TypeTest,
}

pub struct State {
    pub current_view: View,
    pub home: HomeState,
    pub type_test: TypeTestState,
}

impl State {
    pub fn new(keyboard: Keyboard) -> State {
        State {
            current_view: View::TypeTest,
            home: HomeState {},
            type_test: TypeTestState {
                cursor: (0, 0),
                last_modified: None,
                lines: Vec::new(),
                actions: Vec::new(),
                keyboard,
            },
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        // self.actions.push(TimedAction { action, time });

        match self.current_view {
            View::Home => {}
            View::TypeTest => {
                self.type_test.dispatch(TimedAction { action, time });
            }
        }
    }
}
