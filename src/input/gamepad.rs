use fermium::prelude::*;
use glam::Vec2;

use super::ButtonState;

pub(crate) struct Gamepad {
    handle: *mut SDL_GameController,

    pub buttons: ButtonState<GamepadButton>,
    pub left_stick: Vec2,
    pub right_stick: Vec2,
    pub left_trigger: f32,
    pub right_trigger: f32,
}

impl Gamepad {
    pub fn from_raw(raw: *mut SDL_GameController) -> Gamepad {
        Gamepad {
            handle: raw,

            buttons: ButtonState::new(),
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
