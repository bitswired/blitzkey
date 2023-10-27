#[derive(Debug, Clone)]
pub enum Action {
    Char(char),
    Backspace,
    // Enter,
    SetTarget(String),
}
#[derive(Debug, Clone)]
pub struct TimedAction {
    pub action: Action,
    pub time: u128,
}
