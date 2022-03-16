mod generated;

use std::path::Path;

use glam::IVec2;

use crate::graphics::Rectangle;

pub use generated::*;

impl LdtkJson {
    pub fn from_file(path: impl AsRef<Path>) -> LdtkJson {
        let json = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&json).unwrap()
    }

    pub fn get_level(&self, id: &str) -> Option<&Level> {
        self.levels.iter().find(|l| l.identifier == id)
    }
}

impl Level {
    pub fn get_layer(&self, id: &str) -> Option<&LayerInstance> {
        let layers = self.layer_instances.as_ref()?;
        layers.iter().find(|l| l.identifier == id)
    }
}

impl LayerInstance {
    pub fn get_grid_tiles(&self) -> impl Iterator<Item = GridTile> + '_ {
        self.grid_tiles.iter().map(|tile| GridTile {
            position: IVec2::new(tile.px[0] as i32, tile.px[1] as i32),
            uv: Rectangle::new(
                tile.src[0] as f32,
                tile.src[1] as f32,
                self.grid_size as f32,
                self.grid_size as f32,
            ),
            flip_x: tile.f == 1 || tile.f == 3,
            flip_y: tile.f == 2 || tile.f == 3,
        })
    }

    pub fn get_int_grid(&self) -> impl Iterator<Item = IntGridTile> + '_ {
        let width = self.c_wid;

        self.int_grid_csv
            .iter()
            .enumerate()
            .filter_map(move |(i, val)| {
                if *val > 0 {
                    let x = i % width as usize;
                    let y = i / width as usize;

                    Some(IntGridTile {
                        position: IVec2::new(x as i32, y as i32),
                        value: *val as i32,
                    })
                } else {
                    None
                }
            })
    }
}

pub struct GridTile {
    pub position: IVec2,
    pub uv: Rectangle,
    pub flip_x: bool,
    pub flip_y: bool,
}

pub struct IntGridTile {
    pub position: IVec2,
    pub value: i32,
}
