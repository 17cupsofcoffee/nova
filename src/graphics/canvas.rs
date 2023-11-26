use std::rc::Rc;

use glow::HasContext;

use crate::graphics::{Graphics, Texture};

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
    gfx: Graphics,
    pub(crate) id: glow::Framebuffer,
}

impl RawCanvas {
    pub fn new(gfx: &Graphics, texture: &RawTexture) -> RawCanvas {
        unsafe {
            let id = gfx.state.gl.create_framebuffer().unwrap();

            gfx.bind_canvas(Some(id));

            gfx.state.gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture.id),
                0,
            );

            RawCanvas {
                gfx: gfx.clone(),
                id,
            }
        }
    }
}

impl Drop for RawCanvas {
    fn drop(&mut self) {
        unsafe {
            self.gfx.state.gl.delete_framebuffer(self.id);

            if self.gfx.state.current_canvas.get() == Some(self.id) {
                self.gfx.state.current_canvas.set(None);
            }
        }
    }
}
