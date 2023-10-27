use std::collections::HashMap;

use super::actions::{Action, TimedAction};
use super::app_state::StateError;
use crate::keyboard::Keyboard;

pub struct StateStats {
    pub aps: Result<f32, StateError>,
    pub precision: Option<f32>,
    pub words_per_minute: Option<f32>,
    pub keys_precision: Option<Vec<(char, f32)>>,
}
#[derive(PartialEq, Eq)]
pub enum ElementValue {
    Char(char),
    Newline,
}

#[derive(PartialEq)]
pub struct Element {
    pub target: ElementValue,
    pub value: Option<ElementValue>,
}

pub struct Line(pub u8, pub Vec<Element>);

pub type Cursor = (usize, usize);

pub struct TypeTestState {
    pub cursor: Cursor,
    pub lines: Vec<Line>,
    pub actions: Vec<TimedAction>,
    pub keyboard: Keyboard,
    pub last_modified: Option<Cursor>,
}

fn get_lines(target: String) -> Vec<Line> {
    target
        .split("\n")
        .map(|line| {
            let mut trimmed_line = line.trim_start();
            let padding = line.len() - trimmed_line.len();
            trimmed_line = trimmed_line.trim_end();

            let mut elements = Vec::new();
            for c in trimmed_line.chars() {
                elements.push(Element {
                    target: ElementValue::Char(c),
                    value: None,
                });
            }
            elements.push(Element {
                target: ElementValue::Newline,
                value: None,
            });
            Line(padding as u8, elements)
        })
        .collect()
}
impl TypeTestState {
    pub fn dispatch(&mut self, action: TimedAction) {
        match &action.action {
            Action::Char(c) => {
                self.last_modified = Some(self.cursor);

                self.keyboard.key_pressed(*c);

                let line = self.lines.get_mut(self.cursor.0).unwrap();

                let element = line.1.get_mut(self.cursor.1).unwrap();

                match c {
                    '\n' => element.value = Some(ElementValue::Newline),
                    _ => element.value = Some(ElementValue::Char(*c)),
                }

                if self.cursor.1 == line.1.len() - 1 {
                    self.cursor.0 += 1;
                    self.cursor.1 = 0;
                } else {
                    self.cursor.1 += 1;
                }
            }

            Action::Backspace => {
                let element = self
                    .lines
                    .get_mut(self.cursor.0)
                    .unwrap()
                    .1
                    .get_mut(self.cursor.1)
                    .unwrap();
                element.value = None;

                self.last_modified = Some(self.cursor);

                if self.cursor.1 == 0 {
                    self.cursor.0 -= 1;
                    self.cursor.1 = self.lines.get(self.cursor.0).unwrap().1.len() - 1;
                } else {
                    self.cursor.1 -= 1;
                }
            }

            Action::SetTarget(s) => {
                // Trim each line then join and return vec char
                self.lines = get_lines(s.to_string());
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

        let elasped = match self.actions.first() {
            Some(a) => time - a.time,
            None => 0,
        };

        let mut valid = 0;
        let mut total = 0;
        let mut keys_stats: HashMap<char, (i32, i32)> = HashMap::new();

        'outer: for line in &self.lines {
            for element in &line.1 {
                let key = match &element.target {
                    ElementValue::Char(c) => c,
                    ElementValue::Newline => &'\n',
                };

                match (&element.target, &element.value) {
                    (a, Some(b)) if a == b => {
                        valid += 1;
                        keys_stats
                            .entry(*key)
                            .and_modify(|e| e.1 += 1)
                            .or_insert((0, 1));
                    }
                    (_, None) => break 'outer,
                    _ => {}
                }
                total += 1;
                keys_stats
                    .entry(*key)
                    .and_modify(|e| e.0 += 1)
                    .or_insert((1, 0));
            }
        }

        let mut keys_precision: Vec<_> = keys_stats
            .iter()
            .map(|(k, v)| {
                let (total, valid) = v;
                if *total != 0 {
                    (*k, *valid as f32 / *total as f32)
                } else {
                    (*k, 0.0)
                }
            })
            .collect();
        keys_precision.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let precision = if valid == 0 {
            None
        } else {
            Some(valid as f32 / total as f32)
        };

        let words_per_minute = if elasped == 0 {
            None
        } else {
            Some(total as f32 / (elasped) as f32 * 1000.0 * 60.0 / 5.0)
        };

        StateStats {
            aps: self.actions_per_seconds(),
            precision,
            words_per_minute,
            keys_precision: Some(keys_precision),
            // precision: Some(valid as f32),
            // words_per_minute: Some(total as f32),
        }
    }
}
