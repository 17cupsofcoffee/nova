use std::rc::Rc;

use glow::HasContext;

use crate::graphics::{Graphics, State, Texture};

use super::RawTexture;

#[derive(Clone)]
pub struct Canvas {
    pub(crate) raw: Rc<RawCanvas>,
    texture: Texture,
}

impl Canvas {
    pub fn new(gfx: &Graphics, width: i32, height: i32) -> Canvas {
        let texture = Texture::empty(gfx, width, height);
        let raw = RawCanvas::new(gfx, &texture.raw);

        Canvas {
            raw: Rc::new(raw),
            texture,
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

pub struct RawCanvas {
    state: Rc<State>,
    pub(crate) id: glow::Framebuffer,
}

impl RawCanvas {
    pub fn new(gfx: &Graphics, texture: &RawTexture) -> RawCanvas {
        unsafe {
            let id = gfx.state.gl.create_framebuffer().unwrap();

            gfx.state.bind_canvas(Some(id));

            gfx.state.gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture.id),
                0,
            );

            RawCanvas {
                state: Rc::clone(&gfx.state),
                id,
            }
        }
    }
}

impl Drop for RawCanvas {
    fn drop(&mut self) {
        unsafe {
            self.state.gl.delete_framebuffer(self.id);

            if self.state.current_canvas.get() == Some(self.id) {
                self.state.current_canvas.set(None);
            }
        }
    }
}
