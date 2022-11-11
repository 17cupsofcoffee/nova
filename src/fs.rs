use std::path::PathBuf;

use once_cell::sync::OnceCell;
use png::{BitDepth, ColorType, Decoder};

use crate::graphics::{Graphics, Texture};

pub fn base_path() -> &'static PathBuf {
    static BASE_PATH: OnceCell<PathBuf> = OnceCell::new();

    // TODO: Make this use SDL_GetBaseDir when packaging for release
    BASE_PATH.get_or_try_init(std::env::current_dir).unwrap()
}

pub fn asset_path(path: &str) -> PathBuf {
    base_path().join(path)
}

pub fn read(path: &str) -> Vec<u8> {
    let full_path = asset_path(path);

    std::fs::read(full_path).unwrap()
}

pub fn read_to_string(path: &str) -> String {
    let full_path = asset_path(path);

    std::fs::read_to_string(full_path).unwrap()
}

pub fn load_png(gfx: &Graphics, bytes: &[u8], premultiply: bool) -> Texture {
    let decoder = Decoder::new(bytes);
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
