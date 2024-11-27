use std::rc::Rc;

use glow::{HasContext, PixelUnpackData};
use png::{BitDepth, ColorType, Decoder};

use crate::fs;
use crate::graphics::Graphics;

#[derive(Clone)]
pub struct Texture {
    pub(crate) raw: Rc<RawTexture>,
}

impl Texture {
    pub fn from_file(gfx: &Graphics, path: &str, premultiply: bool) -> Texture {
        let bytes = fs::read(path);

        let decoder = Decoder::new(bytes.as_slice());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();

        assert!(info.color_type == ColorType::Rgba);
        assert!(info.bit_depth == BitDepth::Eight);

        if premultiply {
            for pixel in buf.chunks_mut(4) {
                let a = pixel[3];

                if a == 0 {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                } else if a < 255 {
                    pixel[0] = ((pixel[0] as u16 * a as u16) >> 8) as u8;
                    pixel[1] = ((pixel[1] as u16 * a as u16) >> 8) as u8;
                    pixel[2] = ((pixel[2] as u16 * a as u16) >> 8) as u8;
                }
            }
        }

        Texture::from_data(gfx, info.width as i32, info.height as i32, &buf)
    }

    pub fn from_data(gfx: &Graphics, width: i32, height: i32, data: &[u8]) -> Texture {
        let raw = RawTexture::new(gfx, width, height, data);

        Texture { raw: Rc::new(raw) }
    }

    pub fn empty(gfx: &Graphics, width: i32, height: i32) -> Texture {
        Texture::from_data(
            gfx,
            width,
            height,
            &vec![0; width as usize * height as usize * 4],
        )
    }

    pub fn width(&self) -> i32 {
        self.raw.width
    }

    pub fn height(&self) -> i32 {
        self.raw.height
    }

    pub fn size(&self) -> (i32, i32) {
        (self.raw.width, self.raw.height)
    }

    pub fn set_data(&self, data: &[u8]) {
        self.raw
            .set_region(0, 0, self.raw.width, self.raw.height, data);
    }

    pub fn set_region(&self, x: i32, y: i32, width: i32, height: i32, data: &[u8]) {
        self.raw.set_region(x, y, width, height, data);
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.raw.id == other.raw.id
    }
}

pub struct RawTexture {
    gfx: Graphics,
    pub(crate) id: glow::Texture,
    width: i32,
    height: i32,
}

impl RawTexture {
    pub fn new(gfx: &Graphics, width: i32, height: i32, data: &[u8]) -> RawTexture {
        unsafe {
            assert_eq!(width as usize * height as usize * 4, data.len());

            let id = gfx.state.gl.create_texture().unwrap();

            gfx.bind_texture(Some(id));

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
                PixelUnpackData::Slice(Some(data)),
            );

            RawTexture {
                gfx: gfx.clone(),
                id,
                width,
                height,
            }
        }
    }

    pub fn set_region(&self, x: i32, y: i32, width: i32, height: i32, data: &[u8]) {
        unsafe {
            assert_eq!(width as usize * height as usize * 4, data.len());
            assert!(x >= 0 && y >= 0 && x + width <= self.width && y + height <= self.height);

            self.gfx.bind_texture(Some(self.id));

            self.gfx.state.gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                x,
                y,
                width,
                height,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                PixelUnpackData::Slice(Some(data)),
            )
        }
    }
}

impl Drop for RawTexture {
    fn drop(&mut self) {
        unsafe {
            self.gfx.state.gl.delete_texture(self.id);

            if self.gfx.state.current_texture.get() == Some(self.id) {
                self.gfx.state.current_texture.set(None);
            }
        }
    }
}
