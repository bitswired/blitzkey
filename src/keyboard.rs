use std::{collections::HashMap};



use crate::utils::read_file_to_string;

type TouchMap = HashMap<usize, char>;

fn parse_layout(layout: String) -> Result<(String, TouchMap), KeyboardError> {
    let res = layout.split("\n\n").collect::<Vec<&str>>();

    if res.len() != 2 {
        return Err(KeyboardError::LayoutParsingError);
    }

    match (res.first(), res.last()) {
        (Some(layout), Some(mapping)) => {
            let mut touch_map = HashMap::new();

            mapping.chars().enumerate().for_each(|(i, c)| {
                if !" |-_".contains(c) {
                    touch_map.insert(i, c);
                } else {
                    touch_map.insert(i, ' ');
                }
            });

            Ok((String::from(*layout), touch_map))
        }
        _ => Err(KeyboardError::LayoutParsingError),
    }
}

#[derive(Debug)]
pub enum KeyboardError {
    LayoutFileNotFound,
    LayoutParsingError,
}

pub struct Keyboard {
    pub layout: String,
    pub touch_map: TouchMap,
    pub active_keys: HashMap<char, u128>,
}

impl Keyboard {
    pub fn new(layout_path: String) -> Result<Keyboard, KeyboardError> {
        let layout = read_file_to_string(&layout_path);

        match layout {
            Ok(layout) => {
                let (layout, touch_map) = parse_layout(layout)?;
                Ok(Keyboard {
                    layout,
                    touch_map,
                    active_keys: HashMap::new(),
                })
            }
            Err(_) => Err(KeyboardError::LayoutFileNotFound),
        }
    }

    pub fn key_pressed(&mut self, key: char) {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            + 200;

        self.active_keys.insert(key.to_ascii_uppercase(), time);
    }

    pub fn tick(&mut self) {
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut keys_to_remove = Vec::new();

        for (key, time_pressed) in &self.active_keys {
            if time > *time_pressed {
                keys_to_remove.push(*key);
            }
        }

        for key in keys_to_remove {
            self.active_keys.remove(&key);
        }
    }
}
