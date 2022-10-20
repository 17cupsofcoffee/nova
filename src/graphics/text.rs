mod packer;

use std::borrow::Cow;
use std::collections::HashMap;

use fontdue::{Font as FontdueFont, FontSettings};
use glam::Vec2;

use crate::assets;
use crate::graphics::{Color, Graphics, Rectangle, Texture};

use self::packer::ShelfPacker;

const ATLAS_PADDING: i32 = 1;

pub struct Font {
    data: FontdueFont,
}

impl Font {
    pub fn from_file(path: &str) -> Font {
        let bytes = assets::read(path);
        Font::from_data(&bytes)
    }

    pub fn from_data(data: &[u8]) -> Font {
        Font {
            data: FontdueFont::from_bytes(data, FontSettings::default()).unwrap(),
        }
    }
}

pub struct SpriteFontGlyph {
    pub advance: f32,
    pub image: Option<SpriteFontGlyphImage>,
}

pub struct SpriteFontGlyphImage {
    pub offset: Vec2,
    pub uv: Rectangle,
}

pub struct SpriteFont {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,

    texture: Texture,
    cache: HashMap<char, SpriteFontGlyph>,
    kerning: HashMap<(char, char), f32>,
}

impl SpriteFont {
    pub fn new(gfx: &Graphics, font: &Font, size: f32) -> SpriteFont {
        // TODO: Refactor to pack then allocate
        let mut packer = ShelfPacker::new(gfx, 256, 256);
        let mut cache = HashMap::new();
        let mut kerning = HashMap::new();

        let line_metrics = font.data.horizontal_line_metrics(size).unwrap();

        for ch in 32u8..128 {
            let ch = ch as char;

            let (metrics, data) = font.data.rasterize(ch, size);

            let image = if !data.is_empty() {
                let data: Vec<u8> = data.into_iter().flat_map(|x| [x, x, x, x]).collect();

                let uv = packer
                    .insert(
                        &data,
                        metrics.width as i32,
                        metrics.height as i32,
                        ATLAS_PADDING,
                    )
                    .expect("out of space");

                Some(SpriteFontGlyphImage {
                    offset: Vec2::new(
                        metrics.bounds.xmin - ATLAS_PADDING as f32,
                        -metrics.bounds.height - metrics.bounds.ymin - ATLAS_PADDING as f32,
                    ),
                    uv: Rectangle::new(uv.x as f32, uv.y as f32, uv.width as f32, uv.height as f32),
                })
            } else {
                None
            };

            cache.insert(
                ch,
                SpriteFontGlyph {
                    advance: metrics.advance_width,
                    image,
                },
            );

            for ch2 in 32u8..128 {
                let ch2 = ch2 as char;

                if let Some(k) = font.data.horizontal_kern(ch, ch2, size) {
                    kerning.insert((ch, ch2), k);
                }
            }
        }

        SpriteFont {
            ascent: line_metrics.ascent,
            descent: line_metrics.descent,
            line_gap: line_metrics.line_gap,

            texture: packer.into_texture(),
            cache,
            kerning,
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    pub fn glyph(&self, ch: char) -> Option<&SpriteFontGlyph> {
        self.cache.get(&ch)
    }

    pub fn line_height(&self) -> f32 {
        self.ascent - self.descent + self.line_gap
    }

    pub fn kerning(&self, a: char, b: char) -> Option<f32> {
        self.kerning.get(&(a, b)).copied()
    }
}

pub struct TextSegment<'a> {
    pub content: Cow<'a, str>,
    pub color: Color,
}

impl<'a> TextSegment<'a> {
    pub fn new(content: impl Into<Cow<'a, str>>) -> TextSegment<'a> {
        TextSegment {
            content: content.into(),
            color: Color::WHITE,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn into_owned(self) -> TextSegment<'static> {
        TextSegment {
            content: self.content.into_owned().into(),
            color: self.color,
        }
    }
}
