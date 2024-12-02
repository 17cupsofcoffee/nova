use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, Ordering};

use sdl3_sys::error::*;
use sdl3_sys::events::*;
use sdl3_sys::init::*;
use sdl3_sys::keyboard::*;
use sdl3_sys::version::*;
use sdl3_sys::video::*;

use glow::Context;

static SDL_INIT: AtomicBool = AtomicBool::new(false);

pub struct Window {
    window: *mut SDL_Window,
    gl: SDL_GLContext,

    visible: bool,
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32) -> Window {
        unsafe {
            if SDL_INIT.load(Ordering::Relaxed) {
                panic!("SDL already initialized");
            }

            if !SDL_Init(SDL_INIT_VIDEO | SDL_INIT_EVENTS | SDL_INIT_GAMEPAD) {
                sdl_panic!();
            }

            SDL_INIT.store(true, Ordering::Relaxed);

            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG);
            SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);

            let c_title = CString::new(title).unwrap();

            let window = SDL_CreateWindow(
                c_title.as_ptr(),
                width,
                height,
                SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE | SDL_WINDOW_HIDDEN,
            );

            if window.is_null() {
                sdl_panic!();
            }

            SDL_DisableScreenSaver();

            let gl = SDL_GL_CreateContext(window);

            if gl.is_null() {
                sdl_panic!();
            }

            SDL_GL_SetSwapInterval(1);

            let version = SDL_GetVersion();

            println!(
                "SDL Version: {}.{}.{}",
                SDL_VERSIONNUM_MAJOR(version),
                SDL_VERSIONNUM_MINOR(version),
                SDL_VERSIONNUM_MICRO(version),
            );

            Window {
                window,
                gl,

                visible: false,
            }
        }
    }

    pub fn size(&self) -> (u32, u32) {
        unsafe {
            let mut w = 0;
            let mut h = 0;

            SDL_GetWindowSizeInPixels(self.window, &mut w, &mut h);

            (w as u32, h as u32)
        }
    }

    pub fn load_gl(&self) -> Context {
        unsafe {
            Context::from_loader_function(|s| {
                let c_str = CString::new(s).unwrap();
                if let Some(ptr) = SDL_GL_GetProcAddress(c_str.as_ptr()) {
                    ptr as *mut _
                } else {
                    std::ptr::null()
                }
            })
        }
    }

    pub fn next_event(&mut self) -> Option<Event> {
        unsafe {
            let mut raw_event = MaybeUninit::uninit();

            if SDL_PollEvent(raw_event.as_mut_ptr()) {
                let raw_event = raw_event.assume_init();

                Event::try_from_sdl_event(&raw_event)
            } else {
                None
            }
        }
    }

    pub fn present(&mut self) {
        unsafe {
            SDL_GL_SwapWindow(self.window);

            if !self.visible {
                SDL_ShowWindow(self.window);
                self.visible = true;
            }
        }
    }

    pub fn set_title(&mut self, title: &str) {
        let c_title = CString::new(title).unwrap();

        unsafe {
            SDL_SetWindowTitle(self.window, c_title.as_ptr());
        }
    }

    pub fn start_text_input(&mut self) {
        unsafe {
            SDL_StartTextInput(self.window);
        }
    }

    pub fn stop_text_input(&mut self) {
        unsafe {
            SDL_StopTextInput(self.window);
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_GL_DestroyContext(self.gl);
            SDL_DestroyWindow(self.window);
        }
    }
}

pub(crate) unsafe fn get_err() -> String {
    unsafe {
        CStr::from_ptr(SDL_GetError())
            .to_string_lossy()
            .into_owned()
    }
}

macro_rules! sdl_panic {
    () => {
        panic!("SDL error: {}", $crate::window::get_err());
    };
}

pub(crate) use sdl_panic;

use crate::input::Event;
