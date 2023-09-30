use glam::Vec2;

use crate::graphics::Canvas;
use crate::window::Window;

use super::{Batcher, Color, DrawParams, Graphics, Target};

pub struct Scaler {
    canvas: Canvas,

    offset: Vec2,
    scale: Vec2,
}

impl Scaler {
    pub fn new(gfx: &Graphics, width: i32, height: i32) -> Scaler {
        Scaler {
            canvas: Canvas::new(gfx, width, height),

            offset: Vec2::ZERO,
            scale: Vec2::ONE,
        }
    }

    pub fn draw(&mut self, gfx: &Graphics, batch: &mut Batcher, target: &Window) {
        gfx.clear(target, Color::BLACK);

        let (offset, scale) = fit_canvas_to_window(&self.canvas, target);

        batch.texture(
            self.canvas.texture(),
            offset,
            DrawParams::new().scale(scale),
        );

        batch.draw(gfx, target);

        self.offset = offset;
        self.scale = scale;
    }

    pub fn offset(&self) -> Vec2 {
        self.offset
    }

    pub fn scale(&self) -> Vec2 {
        self.scale
    }
}

impl Target for Scaler {
    const FLIPPED: bool = Canvas::FLIPPED;

    fn bind(&self, gfx: &Graphics) {
        self.canvas.bind(gfx)
    }

    fn size(&self) -> (i32, i32) {
        self.canvas.size()
    }
}

pub fn fit_canvas_to_window(canvas: &Canvas, window: &Window) -> (Vec2, Vec2) {
    let (canvas_width, canvas_height) = canvas.size();
    let (window_width, window_height) = window.size();

    let scale = i32::max(
        1,
        i32::min(
            window_width as i32 / canvas_width,
            window_height as i32 / canvas_height,
        ),
    );

    let screen_width = canvas_width * scale;
    let screen_height = canvas_height * scale;
    let screen_x = (window_width as i32 - screen_width) / 2;
    let screen_y = (window_height as i32 - screen_height) / 2;

    (
        Vec2::new(screen_x as f32, screen_y as f32),
        Vec2::splat(scale as f32),
    )
}
