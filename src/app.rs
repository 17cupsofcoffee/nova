use crate::graphics::Graphics;
use crate::input::{Event, Input};
use crate::time::Timer;
use crate::window::Window;

/// The generic event handler for the game. You should implement this yourself
///
/// ## Call order:
/// 1. `event`: 0 to n times based on what events are received
/// 2. `update`: 0 to n times based on the tick rate
/// 3. `draw`: 1 time per frame
pub trait EventHandler {
    /// Handle a raw event for the game. This is useful for one-off events, like key down events.
    /// For continuous events, like moving a character when you hold a key, use the update method.
    ///
    /// note: this will be called after the `app.input` has been updated with the event.
    fn event(&mut self, _app: &mut App, _event: Event) {}
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
    pub fn new(title: &str, width: i32, height: i32, tick_rate: f64) -> App {
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

            self.handle_events(event_handler);

            while self.timer.consume_time() {
                event_handler.update(self);

                self.input.clear();
            }

            event_handler.draw(self);

            self.window.present();
        }
    }

    pub fn handle_events(&mut self, event_handler: &mut impl EventHandler) {
        while let Some(event) = self.window.next_event() {
            if let Some(event) = Event::try_from_sdl_event(&event) {
                if let Event::Quit = event {
                    self.is_running = false;
                }

                self.input.event(&event);

                event_handler.event(self, event);
            }
        }
    }
}
