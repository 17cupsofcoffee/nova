use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};

use fermium::prelude::*;

use glow::Context;

static SDL_INIT: AtomicBool = AtomicBool::new(false);

pub struct Window {
    window: *mut SDL_Window,
    gl: SDL_GLContext,

    visible: bool,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        unsafe {
            if SDL_INIT.load(Ordering::Relaxed) {
                panic!("SDL already initialized");
            }

            if SDL_Init(SDL_INIT_VIDEO | SDL_INIT_EVENTS | SDL_INIT_GAMECONTROLLER) != 0 {
                sdl_panic!();
            }

            SDL_INIT.store(true, Ordering::Relaxed);

            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
            SDL_GL_SetAttribute(
                SDL_GL_CONTEXT_PROFILE_MASK,
                SDL_GL_CONTEXT_PROFILE_CORE.0 as i32,
            );
            SDL_GL_SetAttribute(
                SDL_GL_CONTEXT_FLAGS,
                SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG.0 as i32,
            );
            SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);

            let c_title = CString::new(title).unwrap();

            let window = SDL_CreateWindow(
                c_title.as_ptr(),
                SDL_WINDOWPOS_CENTERED,
                SDL_WINDOWPOS_CENTERED,
                width as i32,
                height as i32,
                (SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE | SDL_WINDOW_HIDDEN).0,
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

            SDL_GL_GetDrawableSize(self.window, &mut w, &mut h);

            (w as u32, h as u32)
        }
    }

    pub fn load_gl(&self) -> Context {
        unsafe {
            Context::from_loader_function(|s| {
                let c_str = CString::new(s).unwrap();
                SDL_GL_GetProcAddress(c_str.as_ptr())
            })
        }
    }

    pub fn next_event(&mut self) -> Option<SDL_Event> {
        unsafe {
            let mut event = SDL_Event::default();

            if SDL_PollEvent(&mut event) == 1 {
                Some(event)
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
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_GL_DeleteContext(self.gl);
            SDL_DestroyWindow(self.window);
        }
    }
}

pub(crate) unsafe fn get_err() -> String {
    let mut v: Vec<u8> = Vec::with_capacity(1024);
    let capacity = v.capacity();

    SDL_GetErrorMsg(v.as_mut_ptr().cast(), capacity.try_into().unwrap());

    let mut len = 0;
    let mut p = v.as_mut_ptr();

    while *p != 0 && len <= capacity {
        p = p.add(1);
        len += 1;
    }

    v.set_len(len);

    match String::from_utf8(v) {
        Ok(s) => s,
        Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    }
}

macro_rules! sdl_panic {
    () => {
        panic!("SDL error: {}", $crate::window::get_err());
    };
}

pub(crate) use sdl_panic;
