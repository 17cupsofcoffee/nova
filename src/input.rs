mod event;
mod gamepad;
mod key;
mod mouse;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use glam::Vec2;

pub use self::event::*;
pub use self::gamepad::*;
pub use self::key::*;
pub use self::mouse::*;

pub struct Input {
    keys: ButtonState<Key>,
    mouse_buttons: ButtonState<MouseButton>,
    gamepad_buttons: ButtonState<(usize, GamepadButton)>,

    axes: AxisState,
    mouse_position: Vec2,

    gamepads: Vec<Option<Gamepad>>,
    joystick_ids: HashMap<JoystickID, usize>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            keys: ButtonState::new(),
            mouse_buttons: ButtonState::new(),
            gamepad_buttons: ButtonState::new(),

            axes: AxisState::new(),
            mouse_position: Vec2::ZERO,

            gamepads: Vec::new(),
            joystick_ids: HashMap::new(),
        }
    }

    pub fn event(&mut self, event: &Event) {
        match event {
            Event::KeyDown(key) => self.keys.set_down(*key),
            Event::KeyUp(key) => self.keys.set_up(*key),
            Event::MouseButtonDown(button) => self.mouse_buttons.set_down(*button),
            Event::MouseButtonUp(button) => self.mouse_buttons.set_up(*button),
            Event::MouseMotion { new_position } => self.mouse_position = *new_position,
            Event::ControllerDeviceAdded { joystick, gamepad } => {
                let empty_slot = self.gamepads.iter().position(Option::is_none);

                let gamepad_id = match empty_slot {
                    Some(slot) => {
                        self.gamepads[slot] = Some(gamepad.clone());
                        slot
                    }
                    None => {
                        self.gamepads.push(Some(gamepad.clone()));
                        self.gamepads.len() - 1
                    }
                };

                self.joystick_ids.insert(*joystick, gamepad_id);
            }
            Event::ControllerDeviceRemoved { joystick } => {
                if let Some(gamepad_id) = self.joystick_ids.remove(joystick) {
                    self.gamepads[gamepad_id] = None;
                }
            }
            Event::ControllerButtonDown { joystick, button } => {
                if let Some(gamepad_id) = self.joystick_ids.get(joystick) {
                    self.gamepad_buttons.set_down((*gamepad_id, *button));
                }
            }
            Event::ControllerButtonUp { joystick, button } => {
                if let Some(gamepad_id) = self.joystick_ids.get(joystick) {
                    self.gamepad_buttons.set_up((*gamepad_id, *button));
                }
            }
            Event::ControllerAxisMotion {
                joystick,
                axis,
                value,
            } => {
                if let Some(gamepad_id) = self.joystick_ids.get(joystick) {
                    self.axes.set_value(*gamepad_id, *axis, *value);
                }
            }
            Event::WindowResized { .. } | Event::TextInput { .. } => {}
        }
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.mouse_buttons.clear();
        self.gamepad_buttons.clear();
        self.axes.clear();
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.keys.is_down(key)
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        self.keys.is_up(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys.is_pressed(key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.keys.is_released(key)
    }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons.is_down(button)
    }

    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        self.mouse_buttons.is_up(button)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.is_pressed(button)
    }

    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        self.mouse_buttons.is_released(button)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn is_gamepad_button_down(&self, player: usize, button: GamepadButton) -> bool {
        self.gamepad_buttons.is_down((player, button))
    }

    pub fn is_gamepad_button_up(&self, player: usize, button: GamepadButton) -> bool {
        self.gamepad_buttons.is_up((player, button))
    }

    pub fn is_gamepad_button_pressed(&self, player: usize, button: GamepadButton) -> bool {
        self.gamepad_buttons.is_pressed((player, button))
    }

    pub fn is_gamepad_button_released(&self, player: usize, button: GamepadButton) -> bool {
        self.gamepad_buttons.is_released((player, button))
    }

    pub fn gamepad_axis(&self, player: usize, axis: GamepadAxis) -> f32 {
        self.axes.get_value(player, axis)
    }

    pub fn has_gamepad_axis_moved(&self, player: usize, axis: GamepadAxis) -> bool {
        self.axes.has_moved(player, axis)
    }

    pub fn gamepad_stick(&self, player: usize, stick: GamepadStick) -> Vec2 {
        let (x, y) = stick.to_axes();

        let x_val = self.axes.get_value(player, x);
        let y_val = self.axes.get_value(player, y);

        Vec2::new(x_val, y_val)
    }

    pub fn has_gamepad_stick_moved(&self, player: usize, stick: GamepadStick) -> bool {
        let (x, y) = stick.to_axes();

        self.axes.has_moved(player, x) || self.axes.has_moved(player, y)
    }
}

pub(crate) struct ButtonState<T> {
    down: HashSet<T>,
    pressed: HashSet<T>,
    released: HashSet<T>,
}

impl<T: Copy + Eq + Hash> ButtonState<T> {
    fn new() -> ButtonState<T> {
        ButtonState {
            down: HashSet::new(),
            pressed: HashSet::new(),
            released: HashSet::new(),
        }
    }

    fn clear(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }

    fn set_down(&mut self, button: T) {
        let was_up = self.down.insert(button);

        if was_up {
            self.pressed.insert(button);
        }
    }

    fn set_up(&mut self, button: T) {
        let was_down = self.down.remove(&button);

        if was_down {
            self.released.insert(button);
        }
    }

    fn is_down(&self, button: T) -> bool {
        self.down.contains(&button)
    }

    fn is_up(&self, button: T) -> bool {
        !self.down.contains(&button)
    }

    fn is_pressed(&self, button: T) -> bool {
        self.pressed.contains(&button)
    }

    fn is_released(&self, button: T) -> bool {
        self.released.contains(&button)
    }
}

pub(crate) struct AxisState {
    curr: HashMap<(usize, GamepadAxis), f32>,
    prev: HashMap<(usize, GamepadAxis), f32>,
}

impl AxisState {
    fn new() -> AxisState {
        AxisState {
            curr: HashMap::new(),
            prev: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.prev = self.curr.clone();
    }

    fn get_value(&self, player: usize, axis: GamepadAxis) -> f32 {
        *self.curr.get(&(player, axis)).unwrap_or(&0.0)
    }

    fn set_value(&mut self, player: usize, axis: GamepadAxis, value: f32) {
        self.curr.insert((player, axis), value);
    }

    fn has_moved(&self, player: usize, axis: GamepadAxis) -> bool {
        self.curr.get(&(player, axis)) != self.prev.get(&(player, axis))
    }
}
