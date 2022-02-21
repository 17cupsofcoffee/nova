use std::rc::Rc;

use glow::HasContext;

use crate::graphics::{Graphics, State, Texture};

pub struct Canvas {
    state: Rc<State>,
    pub(crate) id: glow::Framebuffer,
    texture: Texture,
}

impl Canvas {
    pub fn new(gfx: &Graphics, width: i32, height: i32) -> Canvas {
        unsafe {
            let id = gfx.state.gl.create_framebuffer().unwrap();

            gfx.state.bind_canvas(Some(id));

            let texture = Texture::new(gfx, width, height);

            gfx.state.gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture.inner.id),
                0,
            );

            Canvas {
                state: Rc::clone(&gfx.state),
                id,
                texture,
            }
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn width(&self) -> i32 {
        self.texture.width()
    }

    pub fn height(&self) -> i32 {
        self.texture.height()
    }

    pub fn size(&self) -> (i32, i32) {
        self.texture.size()
    }
}

impl Drop for Canvas {
    fn drop(&mut self) {
        unsafe {
            self.state.gl.delete_framebuffer(self.id);

            if self.state.current_canvas.get() == Some(self.id) {
                self.state.current_canvas.set(None);
            }
        }
    }
}
