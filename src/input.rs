mod gamepad;
mod key;
mod mouse;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use fermium::prelude::*;
use glam::Vec2;

pub use self::gamepad::*;
pub use self::key::*;
pub use self::mouse::*;

pub struct Input {
    buttons: ButtonState,
    axes: AxisState,
    mouse_position: Vec2,

    gamepads: Vec<Option<Gamepad>>,
    joystick_ids: HashMap<SDL_JoystickID, usize>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            buttons: ButtonState::new(),
            axes: AxisState::new(),
            mouse_position: Vec2::ZERO,

            gamepads: Vec::new(),
            joystick_ids: HashMap::new(),
        }
    }

    pub fn event(&mut self, event: &SDL_Event) {
        unsafe {
            match event.type_ {
                SDL_KEYDOWN if event.key.repeat == 0 => {
                    if let Some(key) = Key::from_raw(event.key.keysym.scancode) {
                        self.buttons.set_down(key.into());
                    }
                }

                SDL_KEYUP if event.key.repeat == 0 => {
                    if let Some(key) = Key::from_raw(event.key.keysym.scancode) {
                        self.buttons.set_up(key.into());
                    }
                }

                SDL_MOUSEBUTTONDOWN => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as u32) {
                        self.buttons.set_down(button.into());
                    }
                }

                SDL_MOUSEBUTTONUP => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as u32) {
                        self.buttons.set_up(button.into());
                    }
                }

                SDL_MOUSEMOTION => {
                    self.mouse_position = Vec2::new(event.motion.x as f32, event.motion.y as f32);
                }

                SDL_CONTROLLERDEVICEADDED => {
                    let handle = SDL_GameControllerOpen(event.cdevice.which);

                    if handle.is_null() {
                        // TODO: Should probably log here
                        return;
                    }

                    let joystick = SDL_JoystickInstanceID(SDL_GameControllerGetJoystick(handle));

                    let gamepad = Gamepad::from_raw(handle);

                    let mut empty_slot = None;

                    for (i, slot) in self.gamepads.iter_mut().enumerate() {
                        if slot.is_none() {
                            empty_slot = Some(i);
                            break;
                        }
                    }

                    let gamepad_id = match empty_slot {
                        Some(slot) => {
                            self.gamepads[slot] = Some(gamepad);
                            slot
                        }
                        None => {
                            self.gamepads.push(Some(gamepad));
                            self.gamepads.len() - 1
                        }
                    };

                    self.joystick_ids.insert(joystick, gamepad_id);
                }

                SDL_CONTROLLERDEVICEREMOVED => {
                    if let Some(gamepad_id) = self
                        .joystick_ids
                        .remove(&SDL_JoystickID(event.cdevice.which))
                    {
                        self.gamepads[gamepad_id] = None;
                    }
                }

                SDL_CONTROLLERBUTTONDOWN => {
                    if let Some(button) = GamepadButton::from_raw(SDL_GameControllerButton(
                        event.cbutton.button as i32,
                    )) {
                        if let Some(gamepad_id) = self.joystick_ids.get(&event.cbutton.which) {
                            self.buttons.set_down(button.on(*gamepad_id).into());
                        }
                    }
                }

                SDL_CONTROLLERBUTTONUP => {
                    if let Some(button) = GamepadButton::from_raw(SDL_GameControllerButton(
                        event.cbutton.button as i32,
                    )) {
                        if let Some(gamepad_id) = self.joystick_ids.get(&event.cbutton.which) {
                            self.buttons.set_up(button.on(*gamepad_id).into());
                        }
                    }
                }

                SDL_CONTROLLERAXISMOTION => {
                    if let Some(axis) =
                        GamepadAxis::from_raw(SDL_GameControllerAxis(event.caxis.axis as i32))
                    {
                        if let Some(gamepad_id) = self.joystick_ids.get(&event.cbutton.which) {
                            let mut value = if event.caxis.value > 0 {
                                event.caxis.value as f32 / 32767.0
                            } else {
                                event.caxis.value as f32 / 32768.0
                            };

                            // TODO: Add less hacky deadzone logic
                            if value.abs() < 0.2 {
                                value = 0.0;
                            }

                            self.axes.set_value(axis.on(*gamepad_id), value);
                        }
                    }
                }

                _ => {}
            }
        }
    }

    pub fn clear(&mut self) {
        self.buttons.clear();
        self.axes.clear();
    }

    pub fn is_down(&self, button: impl Into<Button>) -> bool {
        self.buttons.is_down(button.into())
    }

    pub fn is_up(&self, button: impl Into<Button>) -> bool {
        self.buttons.is_up(button.into())
    }

    pub fn is_pressed(&self, button: impl Into<Button>) -> bool {
        self.buttons.is_pressed(button.into())
    }

    pub fn is_released(&self, button: impl Into<Button>) -> bool {
        self.buttons.is_released(button.into())
    }

    pub fn axis(&self, axis: WithGamepadId<GamepadAxis>) -> f32 {
        self.axes.get_value(axis)
    }

    pub fn has_axis_moved(&self, axis: WithGamepadId<GamepadAxis>) -> bool {
        self.axes.has_moved(axis)
    }

    pub fn stick(&self, stick: WithGamepadId<GamepadStick>) -> Vec2 {
        let (x, y) = stick.to_axes();

        let x_val = self.axes.get_value(x);
        let y_val = self.axes.get_value(y);

        Vec2::new(x_val, y_val)
    }

    pub fn has_stick_moved(&self, stick: WithGamepadId<GamepadStick>) -> bool {
        let (x, y) = stick.to_axes();

        self.axes.has_moved(x) || self.axes.has_moved(y)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Key(Key),
    MouseButton(MouseButton),
    GamepadButton(WithGamepadId<GamepadButton>),
}

impl From<Key> for Button {
    fn from(value: Key) -> Self {
        Button::Key(value)
    }
}

impl From<MouseButton> for Button {
    fn from(value: MouseButton) -> Self {
        Button::MouseButton(value)
    }
}

impl From<WithGamepadId<GamepadButton>> for Button {
    fn from(value: WithGamepadId<GamepadButton>) -> Self {
        Button::GamepadButton(value)
    }
}

pub(crate) struct ButtonState {
    down: HashSet<Button>,
    pressed: HashSet<Button>,
    released: HashSet<Button>,
}

impl ButtonState {
    fn new() -> ButtonState {
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

    fn set_down(&mut self, button: Button) {
        let was_up = self.down.insert(button);

        if was_up {
            self.pressed.insert(button);
        }
    }

    fn set_up(&mut self, button: Button) {
        let was_down = self.down.remove(&button);

        if was_down {
            self.released.insert(button);
        }
    }

    fn is_down(&self, button: Button) -> bool {
        self.down.contains(&button)
    }

    fn is_up(&self, button: Button) -> bool {
        !self.down.contains(&button)
    }

    fn is_pressed(&self, button: Button) -> bool {
        self.pressed.contains(&button)
    }

    fn is_released(&self, button: Button) -> bool {
        self.released.contains(&button)
    }
}

pub(crate) struct AxisState {
    curr: HashMap<WithGamepadId<GamepadAxis>, f32>,
    prev: HashMap<WithGamepadId<GamepadAxis>, f32>,
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

    fn get_value(&self, axis: WithGamepadId<GamepadAxis>) -> f32 {
        *self.curr.get(&axis).unwrap_or(&0.0)
    }

    fn set_value(&mut self, axis: WithGamepadId<GamepadAxis>, value: f32) {
        self.curr.insert(axis, value);
    }

    fn has_moved(&self, axis: WithGamepadId<GamepadAxis>) -> bool {
        self.curr.get(&axis) != self.prev.get(&axis)
    }
}
