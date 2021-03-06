use glam::Vec2;

use crate::graphics::Canvas;
use crate::window::Window;

use super::{Batcher, DrawParams};

pub fn screen_scale(canvas: &Canvas, window: &Window, batch: &mut Batcher) {
    let (screen_pos, screen_scale) = fit_canvas_to_window(canvas, window);

    batch.texture(
        canvas.texture(),
        screen_pos,
        DrawParams::new().scale(screen_scale),
    );
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
