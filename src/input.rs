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
    keys: ButtonState<Key>,
    mouse_buttons: ButtonState<MouseButton>,
    mouse_position: Vec2,
    gamepads: Vec<Option<Gamepad>>,
    joystick_ids: HashMap<SDL_JoystickID, usize>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            keys: ButtonState::new(),
            mouse_buttons: ButtonState::new(),
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
                        self.keys.set_down(key);
                    }
                }

                SDL_KEYUP if event.key.repeat == 0 => {
                    if let Some(key) = Key::from_raw(event.key.keysym.scancode) {
                        self.keys.set_up(key);
                    }
                }

                SDL_MOUSEBUTTONDOWN => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as u32) {
                        self.mouse_buttons.set_down(button);
                    }
                }

                SDL_MOUSEBUTTONUP => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as u32) {
                        self.mouse_buttons.set_up(button);
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
                            if let Some(gamepad) = self.get_gamepad_mut(*gamepad_id) {
                                gamepad.buttons.set_down(button);
                            }
                        }
                    }
                }

                SDL_CONTROLLERBUTTONUP => {
                    if let Some(button) = GamepadButton::from_raw(SDL_GameControllerButton(
                        event.cbutton.button as i32,
                    )) {
                        if let Some(gamepad_id) = self.joystick_ids.get(&event.cbutton.which) {
                            if let Some(gamepad) = self.get_gamepad_mut(*gamepad_id) {
                                gamepad.buttons.set_up(button);
                            }
                        }
                    }
                }

                SDL_CONTROLLERAXISMOTION => {
                    if let Some(axis) =
                        GamepadAxis::from_raw(SDL_GameControllerAxis(event.caxis.axis as i32))
                    {
                        if let Some(gamepad_id) = self.joystick_ids.get(&event.caxis.which) {
                            if let Some(gamepad) = self.get_gamepad_mut(*gamepad_id) {
                                let value = if event.caxis.value > 0 {
                                    event.caxis.value as f32 / 32767.0
                                } else {
                                    event.caxis.value as f32 / 32768.0
                                };

                                match axis {
                                    GamepadAxis::LeftStickX => gamepad.left_stick.x = value,
                                    GamepadAxis::LeftStickY => gamepad.left_stick.y = value,
                                    GamepadAxis::RightStickX => gamepad.right_stick.x = value,
                                    GamepadAxis::RightStickY => gamepad.right_stick.y = value,
                                    GamepadAxis::LeftTrigger => gamepad.left_trigger = value,
                                    GamepadAxis::RightTrigger => gamepad.right_trigger = value,
                                }
                            }
                        }
                    }
                }

                _ => {}
            }
        }
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.mouse_buttons.clear();

        for gamepad in self.gamepads.iter_mut().flatten() {
            gamepad.buttons.clear();
        }
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

    pub fn is_mouse_button_down(&self, btn: MouseButton) -> bool {
        self.mouse_buttons.is_down(btn)
    }

    pub fn is_mouse_button_up(&self, btn: MouseButton) -> bool {
        self.mouse_buttons.is_up(btn)
    }

    pub fn is_mouse_button_pressed(&self, btn: MouseButton) -> bool {
        self.mouse_buttons.is_pressed(btn)
    }

    pub fn is_mouse_button_released(&self, btn: MouseButton) -> bool {
        self.mouse_buttons.is_released(btn)
    }

    pub fn is_gamepad_button_down(&self, player: usize, btn: GamepadButton) -> bool {
        self.get_gamepad(player)
            .map(|g| g.buttons.is_down(btn))
            .unwrap_or(false)
    }

    pub fn is_gamepad_button_up(&self, player: usize, btn: GamepadButton) -> bool {
        self.get_gamepad(player)
            .map(|g| g.buttons.is_up(btn))
            .unwrap_or(true)
    }

    pub fn is_gamepad_button_pressed(&self, player: usize, btn: GamepadButton) -> bool {
        self.get_gamepad(player)
            .map(|g| g.buttons.is_pressed(btn))
            .unwrap_or(false)
    }

    pub fn is_gamepad_button_released(&self, player: usize, btn: GamepadButton) -> bool {
        self.get_gamepad(player)
            .map(|g| g.buttons.is_released(btn))
            .unwrap_or(false)
    }

    pub fn gamepad_axis(&self, player: usize, axis: GamepadAxis) -> f32 {
        self.get_gamepad(player)
            .map(|g| match axis {
                GamepadAxis::LeftStickX => g.left_stick.x,
                GamepadAxis::LeftStickY => g.left_stick.y,
                GamepadAxis::RightStickX => g.right_stick.x,
                GamepadAxis::RightStickY => g.right_stick.y,
                GamepadAxis::LeftTrigger => g.left_trigger,
                GamepadAxis::RightTrigger => g.right_trigger,
            })
            .unwrap_or(0.0)
    }

    pub fn gamepad_stick(&self, player: usize, stick: GamepadStick) -> Vec2 {
        self.get_gamepad(player)
            .map(|g| match stick {
                GamepadStick::LeftStick => g.left_stick,
                GamepadStick::RightStick => g.right_stick,
            })
            .unwrap_or(Vec2::ZERO)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    fn get_gamepad(&self, player: usize) -> Option<&Gamepad> {
        self.gamepads.get(player).and_then(|slot| slot.as_ref())
    }

    fn get_gamepad_mut(&mut self, player: usize) -> Option<&mut Gamepad> {
        self.gamepads.get_mut(player).and_then(|slot| slot.as_mut())
    }
}

pub(crate) struct ButtonState<T> {
    down: HashSet<T>,
    pressed: HashSet<T>,
    released: HashSet<T>,
}

impl<T> ButtonState<T>
where
    T: Copy + Eq + Hash,
{
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
