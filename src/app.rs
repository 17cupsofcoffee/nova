use fermium::prelude::SDL_QUIT;

use crate::graphics::Graphics;
use crate::input::Input;
use crate::time::Timer;
use crate::window::Window;

pub trait EventHandler {
    fn update(&mut self, _app: &mut App) {}
    fn draw(&mut self, _app: &mut App) {}
}

pub struct App {
    pub window: Window,
    pub gfx: Graphics,
    pub input: Input,
    pub timer: Timer,

    pub is_running: bool,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32, tick_rate: f64) -> App {
        let mut window = Window::new(title, width, height);
        let gfx = Graphics::new(&mut window);
        let input = Input::new();
        let timer = Timer::new(tick_rate);

        App {
            window,
            gfx,
            input,
            timer,

            is_running: true,
        }
    }

    pub fn run(&mut self, event_handler: &mut impl EventHandler) {
        self.timer.reset();

        while self.is_running {
            self.timer.tick_until_update_ready();

            self.handle_events();

            while self.timer.consume_time() {
                event_handler.update(self);

                self.input.clear();
            }

            event_handler.draw(self);

            self.window.present();
        }
    }

    pub fn handle_events(&mut self) {
        while let Some(event) = self.window.next_event() {
            unsafe {
                if event.type_ == SDL_QUIT {
                    self.is_running = false;
                }
            }

            self.input.event(&event);
        }
    }
}
