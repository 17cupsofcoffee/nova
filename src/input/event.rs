use glam::Vec2;
use sdl3_sys::events::*;
use sdl3_sys::gamepad::*;
use sdl3_sys::joystick::*;

/// This is a unique ID for a joystick for the time it is connected to the
/// system.
///
/// It is never reused for the lifetime of the application. If the joystick is
/// disconnected and reconnected, it will get a new ID.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct JoystickID(SDL_JoystickID);

impl JoystickID {
    fn from_raw(id: SDL_JoystickID) -> JoystickID {
        JoystickID(id)
    }

    fn from_controller_handle(handle: *mut SDL_Gamepad) -> JoystickID {
        unsafe { JoystickID(SDL_GetJoystickID(SDL_GetGamepadJoystick(handle))) }
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
    WindowResized {
        width: u32,
        height: u32,
    },

    TextInput {
        text: String,
    },
}

impl Event {
    pub fn try_from_sdl_event(event: &SDL_Event) -> Option<Self> {
        unsafe {
            match SDL_EventType(event.r#type) {
                SDL_EVENT_KEY_DOWN if !event.key.repeat => {
                    if let Some(key) = Key::from_raw(event.key.scancode) {
                        return Some(Event::KeyDown(key));
                    }
                }

                SDL_EVENT_KEY_UP if !event.key.repeat => {
                    if let Some(key) = Key::from_raw(event.key.scancode) {
                        return Some(Event::KeyUp(key));
                    }
                }

                SDL_EVENT_MOUSE_BUTTON_DOWN => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as i32) {
                        return Some(Event::MouseButtonDown(button));
                    }
                }

                SDL_EVENT_MOUSE_BUTTON_UP => {
                    if let Some(button) = MouseButton::from_raw(event.button.button as i32) {
                        return Some(Event::MouseButtonUp(button));
                    }
                }

                SDL_EVENT_MOUSE_MOTION => {
                    return Some(Event::MouseMotion {
                        new_position: Vec2::new(event.motion.x, event.motion.y),
                    });
                }

                SDL_EVENT_GAMEPAD_ADDED => {
                    let handle = SDL_OpenGamepad(event.cdevice.which);

                    if handle.is_null() {
                        // TODO: Should probably log here
                        return None;
                    }

                    let joystick = JoystickID::from_controller_handle(handle);

                    let gamepad = Gamepad::from_raw(handle);

                    return Some(Event::ControllerDeviceAdded { joystick, gamepad });
                }

                SDL_EVENT_GAMEPAD_REMOVED => {
                    return Some(Event::ControllerDeviceRemoved {
                        joystick: JoystickID::from_raw(event.cdevice.which),
                    });
                }

                SDL_EVENT_GAMEPAD_BUTTON_DOWN => {
                    if let Some(button) =
                        GamepadButton::from_raw(SDL_GamepadButton(event.gbutton.button as i32))
                    {
                        return Some(Event::ControllerButtonDown {
                            joystick: JoystickID::from_raw(event.cdevice.which),
                            button,
                        });
                    }
                }

                SDL_EVENT_GAMEPAD_BUTTON_UP => {
                    if let Some(button) =
                        GamepadButton::from_raw(SDL_GamepadButton(event.gbutton.button as i32))
                    {
                        return Some(Event::ControllerButtonUp {
                            joystick: JoystickID::from_raw(event.cdevice.which),
                            button,
                        });
                    }
                }

                SDL_EVENT_GAMEPAD_AXIS_MOTION => {
                    if let Some(axis) =
                        GamepadAxis::from_raw(SDL_GamepadAxis(event.gaxis.axis as i32))
                    {
                        let mut value = if event.gaxis.value > 0 {
                            event.gaxis.value as f32 / 32767.0
                        } else {
                            event.gaxis.value as f32 / 32768.0
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

                SDL_EVENT_WINDOW_RESIZED => {
                    let e = &event.window;
                    if e.data1 > 0 && e.data2 > 0 {
                        let width = e.data1 as u32;
                        let height = e.data2 as u32;
                        return Some(Event::WindowResized { width, height });
                    }
                }

                SDL_EVENT_TEXT_INPUT => {
                    let text = std::ffi::CStr::from_ptr(event.text.text)
                        .to_string_lossy()
                        .into_owned();
                    return Some(Event::TextInput { text });
                }

                _ => {}
            }
        }
        None
    }
}
