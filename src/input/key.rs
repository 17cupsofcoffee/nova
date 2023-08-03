use fermium::prelude::*;

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

    F1 => SDL_SCANCODE_F1,
    F2 => SDL_SCANCODE_F2,
    F3 => SDL_SCANCODE_F3,
    F4 => SDL_SCANCODE_F4,
    F5 => SDL_SCANCODE_F5,
    F6 => SDL_SCANCODE_F6,
    F7 => SDL_SCANCODE_F7,
    F8 => SDL_SCANCODE_F8,
    F9 => SDL_SCANCODE_F9,
    F10 => SDL_SCANCODE_F10,
    F11 => SDL_SCANCODE_F11,
    F12 => SDL_SCANCODE_F12,
}
