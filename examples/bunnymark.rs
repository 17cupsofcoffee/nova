//! Based on https://github.com/openfl/openfl-samples/tree/master/demos/BunnyMark
//! Original BunnyMark (and sprite) by Iain Lobb

use nova::app::{App, EventHandler};
use nova::graphics::{Batcher, Color, DrawParams, Texture};
use nova::input::{Key, MouseButton};
use nova::math::Vec2;
use rand::rngs::ThreadRng;
use rand::{self, Rng};

const INITIAL_BUNNIES: usize = 1000;
const MAX_X: f32 = 1280.0 - 26.0;
const MAX_Y: f32 = 720.0 - 37.0;
const GRAVITY: f32 = 0.5;

fn main() {
    let mut app = App::new("Bunnymark", 1280, 720, 60.0);
    let mut state = GameState::new(&app);

    app.run(&mut state);
}

struct Bunny {
    position: Vec2,
    velocity: Vec2,
}

impl Bunny {
    fn new(rng: &mut ThreadRng) -> Bunny {
        let x_vel = rng.gen::<f32>() * 5.0;
        let y_vel = (rng.gen::<f32>() * 5.0) - 2.5;

        Bunny {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(x_vel, y_vel),
        }
    }
}

struct GameState {
    batch: Batcher,

    rng: ThreadRng,
    texture: Texture,
    bunnies: Vec<Bunny>,

    auto_spawn: bool,
    spawn_timer: i32,
}

impl GameState {
    fn new(app: &App) -> GameState {
        let mut rng = rand::thread_rng();

        let texture = Texture::from_file(&app.gfx, "examples/wabbit_alpha.png", true);

        let mut bunnies = Vec::with_capacity(INITIAL_BUNNIES);

        for _ in 0..INITIAL_BUNNIES {
            bunnies.push(Bunny::new(&mut rng));
        }

        GameState {
            batch: Batcher::new(&app.gfx),

            rng,
            texture,
            bunnies,

            auto_spawn: false,
            spawn_timer: 0,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, app: &mut App) {
        if self.spawn_timer > 0 {
            self.spawn_timer -= 1;
        }

        if app.input.is_pressed(Key::A) {
            self.auto_spawn = !self.auto_spawn;
        }

        let should_spawn =
            self.spawn_timer == 0 && (app.input.is_down(MouseButton::Left) || self.auto_spawn);

        if should_spawn {
            for _ in 0..INITIAL_BUNNIES {
                self.bunnies.push(Bunny::new(&mut self.rng));
            }
            self.spawn_timer = 10;
        }

        for bunny in &mut self.bunnies {
            bunny.position += bunny.velocity;
            bunny.velocity.y += GRAVITY;

            if bunny.position.x > MAX_X {
                bunny.velocity.x *= -1.0;
                bunny.position.x = MAX_X;
            } else if bunny.position.x < 0.0 {
                bunny.velocity.x *= -1.0;
                bunny.position.x = 0.0;
            }

            if bunny.position.y > MAX_Y {
                bunny.velocity.y *= -0.8;
                bunny.position.y = MAX_Y;

                if self.rng.gen::<bool>() {
                    bunny.velocity.y -= 3.0 + (self.rng.gen::<f32>() * 4.0);
                }
            } else if bunny.position.y < 0.0 {
                bunny.velocity.y = 0.0;
                bunny.position.y = 0.0;
            }
        }

        app.window
            .set_title(&format!("BunnyMark - {} bunnies", self.bunnies.len()));
    }

    fn draw(&mut self, app: &mut App) {
        app.gfx.clear(&app.window, Color::rgb(0.392, 0.584, 0.929));

        for bunny in &self.bunnies {
            self.batch
                .texture(&self.texture, bunny.position, DrawParams::new());
        }

        self.batch.draw(&app.gfx, &app.window);
    }
}
