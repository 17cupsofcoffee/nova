use std::collections::{HashMap, HashSet};

use fermium::prelude::*;
use glam::Vec2;

use crate::window::sdl_panic;

pub struct Input {
    keys_down: HashSet<Key>,
    keys_pressed: HashSet<Key>,
    keys_released: HashSet<Key>,

    mouse_buttons_down: HashSet<MouseButton>,
    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_buttons_released: HashSet<MouseButton>,

    mouse_position: Vec2,

    gamepads: Vec<Option<Gamepad>>,
    joystick_ids: HashMap<SDL_JoystickID, usize>,
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

            gamepads: Vec::new(),
            joystick_ids: HashMap::new(),
        }
    }

    pub fn event(&mut self, event: &SDL_Event) {
        unsafe {
            match event.type_ {
                SDL_KEYDOWN if event.key.repeat == 0 => {
                    if let Some(key) = Key::from_raw(event.key.keysym.scancode) {
                        let was_up = self.keys_down.insert(key);

                        if was_up {
                            self.keys_pressed.insert(key);
                        }
                    }
                }

                SDL_KEYUP if event.key.repeat == 0 => {
                    if let Some(key) = Key::from_raw(event.key.keysym.scancode) {
                        let was_down = self.keys_down.remove(&key);

                        if was_down {
                            self.keys_released.insert(key);
                        }
                    }
                }

                SDL_MOUSEBUTTONDOWN => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as u32) {
                        let was_up = self.mouse_buttons_down.insert(button);

                        if was_up {
                            self.mouse_buttons_pressed.insert(button);
                        }
                    }
                }

                SDL_MOUSEBUTTONUP => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as u32) {
                        let was_down = self.mouse_buttons_down.remove(&button);

                        if was_down {
                            self.mouse_buttons_released.insert(button);
                        }
                    }
                }

                SDL_MOUSEMOTION => {
                    self.mouse_position = Vec2::new(event.motion.x as f32, event.motion.y as f32);
                }

                SDL_CONTROLLERDEVICEADDED => {
                    let handle = SDL_GameControllerOpen(event.cdevice.which);

                    if handle.is_null() {
                        sdl_panic!();
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
                                let was_up = gamepad.buttons_down.insert(button);

                                if was_up {
                                    gamepad.buttons_pressed.insert(button);
                                }
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
                                let was_down = gamepad.buttons_down.remove(&button);

                                if was_down {
                                    gamepad.buttons_released.insert(button);
                                }
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
                                let value = event.caxis.value as f32 / i16::MAX as f32;

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
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_buttons_pressed.clear();
        self.mouse_buttons_released.clear();

        for gamepad in self.gamepads.iter_mut().flatten() {
            gamepad.buttons_pressed.clear();
            gamepad.buttons_released.clear();
        }
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

    pub fn is_gamepad_button_down(&self, player: usize, btn: GamepadButton) -> bool {
        self.get_gamepad(player)
            .map(|g| g.buttons_down.contains(&btn))
            .unwrap_or(false)
    }

    pub fn is_gamepad_button_up(&self, player: usize, btn: GamepadButton) -> bool {
        self.get_gamepad(player)
            .map(|g| !g.buttons_down.contains(&btn))
            .unwrap_or(true)
    }

    pub fn is_gamepad_button_pressed(&self, player: usize, btn: GamepadButton) -> bool {
        self.get_gamepad(player)
            .map(|g| g.buttons_pressed.contains(&btn))
            .unwrap_or(false)
    }

    pub fn is_gamepad_button_released(&self, player: usize, btn: GamepadButton) -> bool {
        self.get_gamepad(player)
            .map(|g| g.buttons_released.contains(&btn))
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

macro_rules! keys {
    ($($key:ident => $raw:ident),*$(,)?) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum Key {
            $($key),*
        }

        impl Key {
            pub(crate) fn from_raw(raw: SDL_Scancode) -> Option<Key> {
                match raw {
                    $($raw => Some(Key::$key)),*,
                    _ => None,
                }
            }
        }
    };
}

keys! {
    Space => SDL_SCANCODE_SPACE,
    Backspace => SDL_SCANCODE_BACKSPACE,
    Enter => SDL_SCANCODE_RETURN,
    Tab => SDL_SCANCODE_TAB,
    CapsLock => SDL_SCANCODE_CAPSLOCK,
    Escape => SDL_SCANCODE_ESCAPE,

    LeftShift => SDL_SCANCODE_LSHIFT,
    LeftCtrl => SDL_SCANCODE_LCTRL,
    LeftAlt => SDL_SCANCODE_LALT,

    Up => SDL_SCANCODE_UP,
    Down => SDL_SCANCODE_DOWN,
    Left => SDL_SCANCODE_LEFT,
    Right => SDL_SCANCODE_RIGHT,

    A => SDL_SCANCODE_A,
    B => SDL_SCANCODE_B,
    C => SDL_SCANCODE_C,
    D => SDL_SCANCODE_D,
    E => SDL_SCANCODE_E,
    F => SDL_SCANCODE_F,
    G => SDL_SCANCODE_G,
    H => SDL_SCANCODE_H,
    I => SDL_SCANCODE_I,
    J => SDL_SCANCODE_J,
    K => SDL_SCANCODE_K,
    L => SDL_SCANCODE_L,
    M => SDL_SCANCODE_M,
    N => SDL_SCANCODE_N,
    O => SDL_SCANCODE_O,
    P => SDL_SCANCODE_P,
    Q => SDL_SCANCODE_Q,
    R => SDL_SCANCODE_R,
    S => SDL_SCANCODE_S,
    T => SDL_SCANCODE_T,
    U => SDL_SCANCODE_U,
    V => SDL_SCANCODE_V,
    W => SDL_SCANCODE_W,
    X => SDL_SCANCODE_X,
    Y => SDL_SCANCODE_Y,
    Z => SDL_SCANCODE_Z,

    Grave => SDL_SCANCODE_GRAVE,
    Num0 => SDL_SCANCODE_0,
    Num1 => SDL_SCANCODE_1,
    Num2 => SDL_SCANCODE_2,
    Num3 => SDL_SCANCODE_3,
    Num4 => SDL_SCANCODE_4,
    Num5 => SDL_SCANCODE_5,
    Num6 => SDL_SCANCODE_6,
    Num7 => SDL_SCANCODE_7,
    Num8 => SDL_SCANCODE_8,
    Num9 => SDL_SCANCODE_9,
    Minus => SDL_SCANCODE_MINUS,
    Equals => SDL_SCANCODE_EQUALS,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

impl MouseButton {
    pub(crate) fn from_raw(raw: u32) -> Option<MouseButton> {
        match raw {
            SDL_BUTTON_LEFT => Some(MouseButton::Left),
            SDL_BUTTON_MIDDLE => Some(MouseButton::Middle),
            SDL_BUTTON_RIGHT => Some(MouseButton::Right),
            SDL_BUTTON_X1 => Some(MouseButton::X1),
            SDL_BUTTON_X2 => Some(MouseButton::X2),
            _ => None,
        }
    }
}

struct Gamepad {
    handle: *mut SDL_GameController,

    buttons_down: HashSet<GamepadButton>,
    buttons_pressed: HashSet<GamepadButton>,
    buttons_released: HashSet<GamepadButton>,

    left_stick: Vec2,
    right_stick: Vec2,
    left_trigger: f32,
    right_trigger: f32,
}

impl Gamepad {
    pub(crate) fn from_raw(raw: *mut SDL_GameController) -> Gamepad {
        Gamepad {
            handle: raw,

            buttons_down: HashSet::new(),
            buttons_pressed: HashSet::new(),
            buttons_released: HashSet::new(),

            left_stick: Vec2::ZERO,
            right_stick: Vec2::ZERO,
            left_trigger: 0.0,
            right_trigger: 0.0,
        }
    }
}

impl Drop for Gamepad {
    fn drop(&mut self) {
        unsafe {
            SDL_GameControllerClose(self.handle);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    Up,
    Down,
    Left,
    Right,
}

impl GamepadButton {
    pub(crate) fn from_raw(raw: SDL_GameControllerButton) -> Option<GamepadButton> {
        match raw {
            SDL_CONTROLLER_BUTTON_A => Some(GamepadButton::A),
            SDL_CONTROLLER_BUTTON_B => Some(GamepadButton::B),
            SDL_CONTROLLER_BUTTON_X => Some(GamepadButton::X),
            SDL_CONTROLLER_BUTTON_Y => Some(GamepadButton::Y),
            SDL_CONTROLLER_BUTTON_BACK => Some(GamepadButton::Back),
            SDL_CONTROLLER_BUTTON_GUIDE => Some(GamepadButton::Guide),
            SDL_CONTROLLER_BUTTON_START => Some(GamepadButton::Start),
            SDL_CONTROLLER_BUTTON_LEFTSTICK => Some(GamepadButton::LeftStick),
            SDL_CONTROLLER_BUTTON_RIGHTSTICK => Some(GamepadButton::RightStick),
            SDL_CONTROLLER_BUTTON_LEFTSHOULDER => Some(GamepadButton::LeftShoulder),
            SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => Some(GamepadButton::RightShoulder),
            SDL_CONTROLLER_BUTTON_DPAD_UP => Some(GamepadButton::Up),
            SDL_CONTROLLER_BUTTON_DPAD_DOWN => Some(GamepadButton::Down),
            SDL_CONTROLLER_BUTTON_DPAD_LEFT => Some(GamepadButton::Left),
            SDL_CONTROLLER_BUTTON_DPAD_RIGHT => Some(GamepadButton::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}

impl GamepadAxis {
    pub(crate) fn from_raw(raw: SDL_GameControllerAxis) -> Option<GamepadAxis> {
        match raw {
            SDL_CONTROLLER_AXIS_LEFTX => Some(GamepadAxis::LeftStickX),
            SDL_CONTROLLER_AXIS_LEFTY => Some(GamepadAxis::LeftStickY),
            SDL_CONTROLLER_AXIS_RIGHTX => Some(GamepadAxis::RightStickX),
            SDL_CONTROLLER_AXIS_RIGHTY => Some(GamepadAxis::RightStickY),
            SDL_CONTROLLER_AXIS_TRIGGERLEFT => Some(GamepadAxis::LeftTrigger),
            SDL_CONTROLLER_AXIS_TRIGGERRIGHT => Some(GamepadAxis::RightTrigger),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GamepadStick {
    LeftStick,
    RightStick,
}
