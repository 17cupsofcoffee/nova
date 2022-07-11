use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Zeroable, Pod)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
    pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    pub fn hex(mut hex: &str) -> Color {
        if hex.starts_with('#') {
            hex = &hex[1..];
        }

        assert!(hex.len() == 6 || hex.len() == 8);

        let r = u32::from_str_radix(&hex[0..2], 16).unwrap() as f32;
        let g = u32::from_str_radix(&hex[2..4], 16).unwrap() as f32;
        let b = u32::from_str_radix(&hex[4..6], 16).unwrap() as f32;

        let a = if hex.len() == 8 {
            u32::from_str_radix(&hex[6..8], 16).unwrap() as f32
        } else {
            255.0
        };

        Color::rgba(r / 255.0, g / 255.0, b / 255.0, a / 255.0)
    }

    // TODO: Not sure if this is the best API
    pub const fn alpha(a: f32) -> Color {
        Color::rgba(a, a, a, a)
    }
}
