use std::{fmt, rc::Rc};

use sdl3_sys::gamepad::*;

#[derive(Clone)]
pub struct Gamepad(#[allow(dead_code)] Rc<GamepadInner>);

impl PartialEq for Gamepad {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for Gamepad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gamepad(...)")
    }
}

struct GamepadInner {
    handle: *mut SDL_Gamepad,
}

impl Gamepad {
    pub fn from_raw(raw: *mut SDL_Gamepad) -> Gamepad {
        Gamepad(Rc::new(GamepadInner { handle: raw }))
    }
}

impl Drop for GamepadInner {
    fn drop(&mut self) {
        unsafe {
            SDL_CloseGamepad(self.handle);
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
    pub(crate) fn from_raw(raw: SDL_GamepadButton) -> Option<GamepadButton> {
        match raw {
            SDL_GAMEPAD_BUTTON_SOUTH => Some(GamepadButton::A),
            SDL_GAMEPAD_BUTTON_EAST => Some(GamepadButton::B),
            SDL_GAMEPAD_BUTTON_WEST => Some(GamepadButton::X),
            SDL_GAMEPAD_BUTTON_NORTH => Some(GamepadButton::Y),
            SDL_GAMEPAD_BUTTON_BACK => Some(GamepadButton::Back),
            SDL_GAMEPAD_BUTTON_GUIDE => Some(GamepadButton::Guide),
            SDL_GAMEPAD_BUTTON_START => Some(GamepadButton::Start),
            SDL_GAMEPAD_BUTTON_LEFT_STICK => Some(GamepadButton::LeftStick),
            SDL_GAMEPAD_BUTTON_RIGHT_STICK => Some(GamepadButton::RightStick),
            SDL_GAMEPAD_BUTTON_LEFT_SHOULDER => Some(GamepadButton::LeftShoulder),
            SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER => Some(GamepadButton::RightShoulder),
            SDL_GAMEPAD_BUTTON_DPAD_UP => Some(GamepadButton::Up),
            SDL_GAMEPAD_BUTTON_DPAD_DOWN => Some(GamepadButton::Down),
            SDL_GAMEPAD_BUTTON_DPAD_LEFT => Some(GamepadButton::Left),
            SDL_GAMEPAD_BUTTON_DPAD_RIGHT => Some(GamepadButton::Right),
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
    pub(crate) fn from_raw(raw: SDL_GamepadAxis) -> Option<GamepadAxis> {
        match raw {
            SDL_GAMEPAD_AXIS_LEFTX => Some(GamepadAxis::LeftStickX),
            SDL_GAMEPAD_AXIS_LEFTY => Some(GamepadAxis::LeftStickY),
            SDL_GAMEPAD_AXIS_RIGHTX => Some(GamepadAxis::RightStickX),
            SDL_GAMEPAD_AXIS_RIGHTY => Some(GamepadAxis::RightStickY),
            SDL_GAMEPAD_AXIS_LEFT_TRIGGER => Some(GamepadAxis::LeftTrigger),
            SDL_GAMEPAD_AXIS_RIGHT_TRIGGER => Some(GamepadAxis::RightTrigger),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GamepadStick {
    LeftStick,
    RightStick,
}

impl GamepadStick {
    pub fn to_axes(&self) -> (GamepadAxis, GamepadAxis) {
        match self {
            GamepadStick::LeftStick => (GamepadAxis::LeftStickX, GamepadAxis::LeftStickY),
            GamepadStick::RightStick => (GamepadAxis::RightStickX, GamepadAxis::RightStickY),
        }
    }
}
