use sdl3_sys::mouse::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
}

impl MouseButton {
    pub(crate) fn from_raw(raw: i32) -> Option<MouseButton> {
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
