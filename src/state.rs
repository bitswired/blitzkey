pub enum StateError {
    NoActionYet,
}

#[derive(Debug, Clone)]
pub enum Action {
    Char(char),
    Backspace,
    Enter,
    SetTarget(String),
}
#[derive(Debug, Clone)]
pub struct TimedAction {
    action: Action,
    time: u128,
}

pub struct HomeState {}
pub struct TypeTestState {
    pub cursor: usize,
    pub target: Vec<char>,
    pub player_moves: Vec<char>,
    pub actions: Vec<TimedAction>,
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

pub struct StateStats {
    pub aps: Result<f32, StateError>,
    pub precision: Option<f32>,
    pub words_per_minute: Option<f32>,
}

impl TypeTestState {
    pub fn dispatch(&mut self, action: TimedAction) {
        match &action.action {
            Action::Char(c) => {
                self.cursor += 1;
                self.player_moves.push(*c);
            }

            Action::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.player_moves.pop();
                }
            }

            Action::Enter => {
                self.cursor += 1;
                self.player_moves.push('\n');
            }

            Action::SetTarget(s) => {
                // Trim each line then join and return vec char
                self.target = s
                    .lines()
                    .map(|l| l.trim_end())
                    .collect::<Vec<&str>>()
                    .join("\n")
                    .chars()
                    .collect();
            }
        }

        self.actions.push(action);
    }

    pub fn actions_per_seconds(&self) -> Result<f32, StateError> {
        if self.actions.is_empty() {
            return Err(StateError::NoActionYet);
        }

        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let elapsed = time - self.actions.first().unwrap().time;

        if elapsed == 0 {
            return Err(StateError::NoActionYet);
        }

        let aps = self.actions.len() as f32 / (elapsed as f32 / 1000.0);

        Ok(aps)
    }

    pub fn stats(&self) -> StateStats {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let precision = match self.player_moves.len() {
            0 => None,
            _ => Some(
                self.player_moves
                    .iter()
                    .zip(self.target.iter())
                    .fold(0, |acc, (a, b)| if a == b { acc + 1 } else { acc })
                    as f32
                    / self.player_moves.len() as f32,
            ),
        };

        let words_per_minute = match (self.actions.first(), self.actions.last()) {
            (Some(a1), Some(a2)) if a1.time != a2.time => {
                Some(self.player_moves.len() as f32 / (time - a1.time) as f32 * 1000.0 * 60.0 / 5.0)
            }
            _ => None,
        };

        StateStats {
            aps: self.actions_per_seconds(),
            precision,
            words_per_minute,
        }
    }
}

impl State {
    pub fn new() -> State {
        State {
            current_view: View::TypeTest,
            home: HomeState {},
            type_test: TypeTestState {
                cursor: 0,
                target: Vec::new(),
                player_moves: Vec::new(),
                actions: Vec::new(),
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
