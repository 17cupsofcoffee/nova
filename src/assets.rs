use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::time::Instant;

use png::{BitDepth, ColorType, Decoder};

use crate::graphics::{Graphics, Texture};

pub fn load_assets<T>(
    path: impl AsRef<Path>,
    ext: &str,
    mut loader: impl FnMut(&[u8]) -> T,
) -> HashMap<String, T> {
    let start = Instant::now();

    let path = path.as_ref();
    let mut assets = HashMap::new();

    for entry in std::fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();

        let file_name = path.file_name().and_then(OsStr::to_str).unwrap();
        let file_stem = path.file_stem().and_then(OsStr::to_str).unwrap();

        if file_name.ends_with(ext) {
            let bytes = std::fs::read(&path).unwrap();
            let asset = loader(&bytes);

            assets.insert(file_stem.to_owned(), asset);
        }
    }

    let end = Instant::now();

    println!("Loading {} took {:?}", path.display(), end - start);

    assets
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

    Texture::new(gfx, info.width as i32, info.height as i32, &buf)
}
