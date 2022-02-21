use glow::Context;
use sdl2::video::{GLContext, SwapInterval, Window as SdlWindow};
use sdl2::{EventPump, Sdl, VideoSubsystem};

pub use sdl2::event::Event;

pub struct Window {
    _sdl: Sdl,
    video: VideoSubsystem,
    event_pump: EventPump,
    window: SdlWindow,
    _gl: GLContext,

    visible: bool,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Window {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let event_pump = sdl.event_pump().unwrap();

        let gl_attr = video.gl_attr();
        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_flags().forward_compatible().set();
        gl_attr.set_double_buffer(true);

        let window = video
            .window(title, width, height)
            .position_centered()
            .opengl()
            .resizable()
            .hidden()
            .build()
            .unwrap();

        video.disable_screen_saver();

        let _gl = window.gl_create_context().unwrap();

        let _ = video.gl_set_swap_interval(SwapInterval::VSync);

        Window {
            _sdl: sdl,
            video,
            event_pump,
            window,
            _gl,

            visible: false,
        }
    }

    pub fn size(&self) -> (u32, u32) {
        self.window.drawable_size()
    }

    pub fn load_gl(&self) -> Context {
        unsafe { Context::from_loader_function(|s| self.video.gl_get_proc_address(s) as *const _) }
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.event_pump.poll_event()
    }

    pub fn present(&mut self) {
        self.window.gl_swap_window();

        if !self.visible {
            self.window.show();
            self.visible = true;
        }
    }
}
