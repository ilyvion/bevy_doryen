use crate::doryen::InputApi;
use std::collections::{HashMap, HashSet};
use std::iter::Filter;

#[derive(Default)]
pub struct DoryenInput {
    keys_down: HashMap<String, bool>,
    keys_pressed: HashMap<String, bool>,
    keys_released: HashMap<String, bool>,
    mouse_buttons_down: HashMap<usize, bool>,
    mouse_buttons_pressed: HashSet<usize>,
    mouse_buttons_released: HashSet<usize>,
    text: String,
    close_requested: bool,
    mouse_position: (f32, f32),
}

type KeyMapFilter<'a> =
    Filter<std::collections::hash_map::Iter<'a, String, bool>, fn(&(&'a String, &'a bool)) -> bool>;

/// An iterator visiting all keys in arbitrary order.
pub struct Keys<'a> {
    inner: KeyMapFilter<'a>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k.as_ref())
    }
}

impl DoryenInput {
    fn clear(&mut self) {
        for (_, v) in self.keys_pressed.iter_mut() {
            *v = false;
        }
        for (_, v) in self.keys_released.iter_mut() {
            *v = false;
        }
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
        self.close_requested = false;
    }

    pub(crate) fn handle_input(
        &mut self,
        mouse_button_listeners: &[usize],
        input: &mut dyn InputApi,
    ) {
        self.clear();
        for k in input.keys_pressed() {
            if let Some(v) = self.keys_pressed.get_mut(k) {
                *v = true;
            } else {
                self.keys_pressed.insert(String::from(k), true);
            }

            if let Some(v) = self.keys_down.get_mut(k) {
                *v = true;
            } else {
                self.keys_down.insert(String::from(k), true);
            }
        }
        for k in input.keys_released() {
            if let Some(v) = self.keys_released.get_mut(k) {
                *v = true;
            } else {
                self.keys_released.insert(String::from(k), true);
            }

            if let Some(v) = self.keys_down.get_mut(k) {
                *v = false;
            } else {
                self.keys_down.insert(String::from(k), false);
            }
        }
        for &m in mouse_button_listeners {
            if input.mouse_button_pressed(m) {
                self.mouse_buttons_pressed.insert(m);
                let down = self.mouse_buttons_down.entry(m).or_default();
                *down = true;
            }
            if input.mouse_button_released(m) {
                self.mouse_buttons_released.insert(m);
                let down = self.mouse_buttons_down.entry(m).or_default();
                *down = false;
            }
        }
        self.text = input.text();
        self.mouse_position = input.mouse_pos();
        self.close_requested = input.close_requested();
    }

    pub fn key(&self, scan_code: &str) -> bool {
        matches!(self.keys_down.get(scan_code), Some(&true))
    }

    pub fn key_pressed(&self, scan_code: &str) -> bool {
        matches!(self.keys_pressed.get(scan_code), Some(&true))
    }

    pub fn keys_pressed(&self) -> Keys {
        Keys {
            inner: self.keys_pressed.iter().filter(|&(_, &v)| v),
        }
    }

    pub fn key_released(&self, scan_code: &str) -> bool {
        matches!(self.keys_released.get(scan_code), Some(&true))
    }

    pub fn keys_released(&self) -> Keys {
        Keys {
            inner: self.keys_released.iter().filter(|&(_, &v)| v),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn mouse_button(&self, num: usize) -> bool {
        matches!(self.mouse_buttons_down.get(&num), Some(&true))
    }

    pub fn mouse_button_pressed(&self, num: usize) -> bool {
        self.mouse_buttons_pressed.contains(&num)
    }

    pub fn mouse_button_released(&self, num: usize) -> bool {
        self.mouse_buttons_released.contains(&num)
    }

    pub fn mouse_pos(&self) -> (f32, f32) {
        self.mouse_position
    }

    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
}
