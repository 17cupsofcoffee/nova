use fermium::prelude::*;
use glam::Vec2;

/// This is a unique ID for a joystick for the time it is connected to the
/// system.
///
/// It is never reused for the lifetime of the application. If the joystick is
/// disconnected and reconnected, it will get a new ID.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct JoystickID(SDL_JoystickID);

impl JoystickID {
    fn from_raw(id: i32) -> JoystickID {
        JoystickID(SDL_JoystickID(id))
    }
    fn from_controller_handle(handle: *mut SDL_GameController) -> JoystickID {
        unsafe {
            JoystickID(SDL_JoystickInstanceID(SDL_GameControllerGetJoystick(
                handle,
            )))
        }
    }
}

use super::{Gamepad, GamepadAxis, GamepadButton, Key, MouseButton};

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    KeyDown(Key),
    KeyUp(Key),
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    MouseMotion {
        new_position: Vec2,
    },
    ControllerDeviceAdded {
        joystick: JoystickID,
        gamepad: Gamepad,
    },
    ControllerDeviceRemoved {
        joystick: JoystickID,
    },

    ControllerButtonDown {
        joystick: JoystickID,
        button: GamepadButton,
    },
    ControllerButtonUp {
        joystick: JoystickID,
        button: GamepadButton,
    },

    ControllerAxisMotion {
        joystick: JoystickID,
        axis: GamepadAxis,
        value: f32,
    },
}

impl Event {
    pub fn try_from_sdl_event(event: &SDL_Event) -> Option<Self> {
        unsafe {
            match event.type_ {
                SDL_KEYDOWN if event.key.repeat == 0 => {
                    if let Some(key) = Key::from_raw(event.key.keysym.scancode) {
                        return Some(Event::KeyDown(key));
                    }
                }

                SDL_KEYUP if event.key.repeat == 0 => {
                    if let Some(key) = Key::from_raw(event.key.keysym.scancode) {
                        return Some(Event::KeyUp(key));
                    }
                }

                SDL_MOUSEBUTTONDOWN => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as u32) {
                        return Some(Event::MouseButtonDown(button));
                    }
                }

                SDL_MOUSEBUTTONUP => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as u32) {
                        return Some(Event::MouseButtonUp(button));
                    }
                }

                SDL_MOUSEMOTION => {
                    return Some(Event::MouseMotion {
                        new_position: Vec2::new(event.motion.x as f32, event.motion.y as f32),
                    });
                }

                SDL_CONTROLLERDEVICEADDED => {
                    let handle = SDL_GameControllerOpen(event.cdevice.which);

                    if handle.is_null() {
                        // TODO: Should probably log here
                        return None;
                    }

                    let joystick = JoystickID::from_controller_handle(handle);

                    let gamepad = Gamepad::from_raw(handle);

                    return Some(Event::ControllerDeviceAdded { joystick, gamepad });
                }

                SDL_CONTROLLERDEVICEREMOVED => {
                    return Some(Event::ControllerDeviceRemoved {
                        joystick: JoystickID::from_raw(event.cdevice.which),
                    });
                }

                SDL_CONTROLLERBUTTONDOWN => {
                    if let Some(button) = GamepadButton::from_raw(SDL_GameControllerButton(
                        event.cbutton.button as i32,
                    )) {
                        return Some(Event::ControllerButtonDown {
                            joystick: JoystickID::from_raw(event.cdevice.which),
                            button,
                        });
                    }
                }

                SDL_CONTROLLERBUTTONUP => {
                    if let Some(button) = GamepadButton::from_raw(SDL_GameControllerButton(
                        event.cbutton.button as i32,
                    )) {
                        return Some(Event::ControllerButtonUp {
                            joystick: JoystickID::from_raw(event.cdevice.which),
                            button,
                        });
                    }
                }

                SDL_CONTROLLERAXISMOTION => {
                    if let Some(axis) =
                        GamepadAxis::from_raw(SDL_GameControllerAxis(event.caxis.axis as i32))
                    {
                        let mut value = if event.caxis.value > 0 {
                            event.caxis.value as f32 / 32767.0
                        } else {
                            event.caxis.value as f32 / 32768.0
                        };

                        // TODO: Add less hacky deadzone logic
                        if value.abs() < 0.2 {
                            value = 0.0;
                        }
                        return Some(Event::ControllerAxisMotion {
                            joystick: JoystickID::from_raw(event.cdevice.which),
                            axis,
                            value,
                        });
                    }
                }

                _ => {}
            }
        }
        None
    }
}
