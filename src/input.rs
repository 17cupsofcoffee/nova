use std::collections::HashSet;

use glam::Vec2;
use sdl2::event::Event;

pub use sdl2::keyboard::Scancode as Key;
pub use sdl2::mouse::MouseButton;

pub struct Input {
    keys_down: HashSet<Key>,
    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,

    mouse_buttons_down: HashSet<MouseButton>,
    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_buttons_released: HashSet<MouseButton>,

    mouse_position: Vec2,
}

impl Input {
    pub fn new() -> Input {
        Input {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),

            mouse_buttons_down: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_released: HashSet::new(),

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

            Event::MouseButtonDown { mouse_btn, .. } => {
                let was_up = self.mouse_buttons_down.insert(*mouse_btn);

                if was_up {
                    self.mouse_buttons_pressed.insert(*mouse_btn);
                }
            }

            Event::MouseButtonUp { mouse_btn, .. } => {
                let was_down = self.mouse_buttons_down.remove(mouse_btn);

                if was_down {
                    self.mouse_buttons_released.insert(*mouse_btn);
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
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        !self.keys_down.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys_released.contains(&key)
    }

    pub fn is_mouse_button_down(&self, btn: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&btn)
    }

    pub fn is_mouse_button_up(&self, btn: MouseButton) -> bool {
        !self.mouse_buttons_down.contains(&btn)
    }

    pub fn is_mouse_button_pressed(&self, btn: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&btn)
    }

    pub fn is_mouse_button_released(&self, btn: MouseButton) -> bool {
        self.mouse_buttons_released.contains(&btn)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
}
