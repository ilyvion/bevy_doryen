use crate::doryen::InputApi;
use std::collections::{HashMap, HashSet};
use std::iter::Filter;

/// Provides access to the input events handled by the Doryen engine. See the
/// documentation for the [`InputApi`] type for details on what values should
/// be used with the various `key` methods.
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
        mouse_button_listeners: &[MouseButton],
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
        for &mouse_button in mouse_button_listeners {
            let mouse_button_num = mouse_button.to_usize();
            if input.mouse_button_pressed(mouse_button_num) {
                self.mouse_buttons_pressed.insert(mouse_button_num);
                let down = self.mouse_buttons_down.entry(mouse_button_num).or_default();
                *down = true;
            }
            if input.mouse_button_released(mouse_button_num) {
                self.mouse_buttons_released.insert(mouse_button_num);
                let down = self.mouse_buttons_down.entry(mouse_button_num).or_default();
                *down = false;
            }
        }
        self.text = input.text();
        self.mouse_position = input.mouse_pos();
        self.close_requested = input.close_requested();
    }

    /// Returns the current status of the given key (true if currently pressed).
    pub fn key(&self, key: &str) -> bool {
        matches!(self.keys_down.get(key), Some(&true))
    }

    /// Returns true if the given key was pressed since the last update.
    pub fn key_pressed(&self, key: &str) -> bool {
        matches!(self.keys_pressed.get(key), Some(&true))
    }

    /// Returns an iterator over all the keys that were pressed since the last
    /// update in no particular order.
    pub fn keys_pressed(&self) -> Keys {
        Keys {
            inner: self.keys_pressed.iter().filter(|&(_, &v)| v),
        }
    }

    /// Returns true if the given key was released since the last update.
    pub fn key_released(&self, key: &str) -> bool {
        matches!(self.keys_released.get(key), Some(&true))
    }

    /// Returns an iterator over all the keys that were released since the last
    /// update in no particular order.
    pub fn keys_released(&self) -> Keys {
        Keys {
            inner: self.keys_released.iter().filter(|&(_, &v)| v),
        }
    }

    /// Characters typed since last update.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the current status of the given mouse button (true if
    /// currently pressed).
    pub fn mouse_button(&self, mouse_button: MouseButton) -> bool {
        matches!(
            self.mouse_buttons_down.get(&mouse_button.to_usize()),
            Some(&true)
        )
    }

    /// Returns true if the given mouse button was pressed since the last
    /// update.
    pub fn mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        self.mouse_buttons_pressed
            .contains(&mouse_button.to_usize())
    }

    /// Returns true if the given mouse button was released since the last
    /// update.
    pub fn mouse_button_released(&self, mouse_button: MouseButton) -> bool {
        self.mouse_buttons_released
            .contains(&mouse_button.to_usize())
    }

    /// Returns the current mouse position in console cells coordinates.
    /// The decimal part of the value indicates sub-cell location.
    pub fn mouse_pos(&self) -> (f32, f32) {
        self.mouse_position
    }

    /// Whether the window close button has been activated.
    pub fn close_requested(&self) -> bool {
        self.close_requested
    }
}

/// Represents buttons on a mouse.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    /// Represents the left button on a mouse.
    Left,
    /// Represents the middle button on a mouse.
    Middle,
    /// Represents the right button on a mouse.
    Right,
    /// Represents any given button on a mouse.
    Any(usize),
}

impl MouseButton {
    #[inline]
    fn to_usize(&self) -> usize {
        match self {
            MouseButton::Left => 0,
            MouseButton::Middle => 1,
            MouseButton::Right => 2,
            MouseButton::Any(which) => *which,
        }
    }
}
