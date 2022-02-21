use std::collections::HashSet;

use glam::Vec2;
use sdl2::event::Event;

pub use sdl2::keyboard::Scancode as Key;

pub struct Input {
    keys_down: HashSet<Key>,
    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,

    mouse_position: Vec2,
}

impl Input {
    pub fn new() -> Input {
        Input {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),

            mouse_position: Vec2::ZERO,
        }
    }

    pub fn event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                scancode: Some(scancode),
                repeat: false,
                ..
            } => {
                let was_up = self.keys_down.insert(*scancode);

                if was_up {
                    self.keys_pressed.insert(*scancode);
                }
            }

            Event::KeyUp {
                scancode: Some(scancode),
                repeat: false,
                ..
            } => {
                let was_down = self.keys_down.remove(scancode);

                if was_down {
                    self.keys_released.insert(*scancode);
                }
            }

            Event::MouseMotion { x, y, .. } => {
                self.mouse_position = Vec2::new(*x as f32, *y as f32);
            }

            _ => {}
        }
    }

    pub fn clear(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
    }

    pub fn is_down(&self, key: Key) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_up(&self, key: Key) -> bool {
        !self.keys_down.contains(&key)
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
}
