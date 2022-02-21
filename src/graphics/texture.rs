use std::rc::Rc;

use glow::{HasContext, PixelUnpackData};

use crate::graphics::{Graphics, State};

pub struct TextureInner {
    state: Rc<State>,
    pub(crate) id: glow::Texture,
    width: i32,
    height: i32,
}

impl Drop for TextureInner {
    fn drop(&mut self) {
        unsafe {
            self.state.gl.delete_texture(self.id);

            if self.state.current_texture.get() == Some(self.id) {
                self.state.current_texture.set(None);
            }
        }
    }
}

#[derive(Clone)]
pub struct Texture {
    pub(crate) inner: Rc<TextureInner>,
}

impl Texture {
    pub fn new(gfx: &Graphics, width: i32, height: i32) -> Texture {
        unsafe {
            let id = gfx.state.gl.create_texture().unwrap();

            gfx.state.bind_texture(Some(id));

            gfx.state.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );

            gfx.state.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );

            gfx.state.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );

            gfx.state.gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );

            gfx.state
                .gl
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_BASE_LEVEL, 0);

            gfx.state
                .gl
                .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAX_LEVEL, 0);

            gfx.state.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );

            Texture {
                inner: Rc::new(TextureInner {
                    state: Rc::clone(&gfx.state),
                    id,
                    width,
                    height,
                }),
            }
        }
    }

    pub fn width(&self) -> i32 {
        self.inner.width
    }

    pub fn height(&self) -> i32 {
        self.inner.height
    }

    pub fn size(&self) -> (i32, i32) {
        (self.inner.width, self.inner.height)
    }

    pub fn set_data(&self, data: &[u8]) {
        self.set_region(0, 0, self.inner.width, self.inner.height, data);
    }

    pub fn set_region(&self, x: i32, y: i32, width: i32, height: i32, data: &[u8]) {
        unsafe {
            // TODO: checks

            self.inner.state.bind_texture(Some(self.inner.id));

            self.inner.state.gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                x,
                y,
                width,
                height,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                PixelUnpackData::Slice(data),
            )
        }
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}
