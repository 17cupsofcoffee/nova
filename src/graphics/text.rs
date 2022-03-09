mod packer;

use std::collections::HashMap;

use fontdue::{Font as FontdueFont, FontSettings};
use glam::Vec2;

use crate::graphics::{Color, Graphics, Rectangle, Texture};

use self::packer::ShelfPacker;

const ATLAS_PADDING: i32 = 1;

pub struct Font {
    data: FontdueFont,
}

impl Font {
    pub fn new(data: &[u8]) -> Font {
        Font {
            data: FontdueFont::from_bytes(data, FontSettings::default()).unwrap(),
        }
    }
}

pub struct SpriteFontGlyph {
    pub advance: f32,
    pub offset: Vec2,
    pub uv: Rectangle,
}

pub struct SpriteFont {
    ascent: f32,
    descent: f32,
    line_height: f32,
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
            let data: Vec<u8> = data.into_iter().map(|x| [x, x, x, x]).flatten().collect();

            let uv = packer
                .insert(
                    &data,
                    metrics.width as i32,
                    metrics.height as i32,
                    ATLAS_PADDING,
                )
                .expect("out of space");

            cache.insert(
                ch,
                SpriteFontGlyph {
                    advance: metrics.advance_width,
                    offset: Vec2::new(
                        metrics.bounds.xmin - ATLAS_PADDING as f32,
                        -metrics.bounds.height - metrics.bounds.ymin - ATLAS_PADDING as f32,
                    ),
                    uv: Rectangle::new(uv.x as f32, uv.y as f32, uv.width as f32, uv.height as f32),
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
            line_height: line_metrics.new_line_size,
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

    pub fn ascent(&self) -> f32 {
        self.ascent
    }

    pub fn descent(&self) -> f32 {
        self.descent
    }

    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    pub fn kerning(&self, a: char, b: char) -> Option<f32> {
        self.kerning.get(&(a, b)).copied()
    }
}

#[derive(Clone, Debug)]
pub enum TextSection {
    String(String),
    ChangeColor(Color),
}

#[derive(Clone, Debug)]
pub struct RichText {
    pub sections: Vec<TextSection>,
}

impl RichText {
    pub fn new(sections: impl Into<Vec<TextSection>>) -> RichText {
        RichText {
            sections: sections.into(),
        }
    }

    pub fn text(&self) -> impl Iterator<Item = &str> + '_ {
        self.sections.iter().filter_map(|section| match section {
            TextSection::String(s) => Some(s.as_str()),
            _ => None,
        })
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.text().flat_map(str::chars)
    }
}
