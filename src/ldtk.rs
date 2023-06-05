use std::path::Path;

use glam::{BVec2, IVec2};
use serde::{Deserialize, Deserializer};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub levels: Vec<Level>,
    pub defs: Defs,
}

impl Project {
    pub fn from_file(path: impl AsRef<Path>) -> Project {
        let json = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&json).unwrap()
    }

    pub fn get_level(&self, id: &str) -> Option<&Level> {
        self.levels.iter().find(|l| l.identifier == id)
    }

    pub fn get_level_by_iid(&self, id: &str) -> Option<&Level> {
        self.levels.iter().find(|l| l.iid == id)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defs {
    pub tilesets: Vec<Tileset>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tileset {
    pub enum_tags: Vec<EnumTag>,
}

impl Tileset {
    pub fn get_enum_tag(&self, id: &str) -> Option<&EnumTag> {
        self.enum_tags.iter().find(|l| l.enum_value_id == id)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnumTag {
    pub enum_value_id: String,
    pub tile_ids: Vec<i32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    #[serde(rename = "__neighbours")]
    pub neighbours: Vec<Neighbour>,

    pub identifier: String,

    pub iid: String,

    #[serde(default = "Vec::new")]
    pub layer_instances: Vec<LayerInstance>,
}

impl Level {
    pub fn get_layer(&self, id: &str) -> Option<&LayerInstance> {
        self.layer_instances.iter().find(|l| l.identifier == id)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Neighbour {
    pub dir: NeighbourDirection,
    pub level_iid: String,
}

#[derive(Clone, Debug, Deserialize)]
pub enum NeighbourDirection {
    #[serde(rename = "n")]
    North,

    #[serde(rename = "s")]
    South,

    #[serde(rename = "e")]
    East,

    #[serde(rename = "w")]
    West,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayerInstance {
    #[serde(rename = "__cWid")]
    pub width: i32,

    #[serde(rename = "__cHei")]
    pub height: i32,

    #[serde(rename = "__gridSize")]
    pub grid_size: i32,

    #[serde(rename = "__identifier")]
    pub identifier: String,

    pub entity_instances: Vec<EntityInstance>,

    pub grid_tiles: Vec<GridTile>,

    pub int_grid_csv: Vec<i32>,
}

impl LayerInstance {
    pub fn get_int_grid(&self) -> impl Iterator<Item = IntGridTile> + '_ {
        let width = self.width;

        self.int_grid_csv
            .iter()
            .enumerate()
            .filter_map(move |(i, val)| {
                if *val > 0 {
                    let x = i % width as usize;
                    let y = i / width as usize;

                    Some(IntGridTile {
                        position: IVec2::new(x as i32, y as i32),
                        value: *val,
                    })
                } else {
                    None
                }
            })
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GridTile {
    #[serde(rename = "t")]
    pub id: i32,

    #[serde(deserialize_with = "deserialize_ldtk_point")]
    pub px: IVec2,

    #[serde(deserialize_with = "deserialize_ldtk_point")]
    pub src: IVec2,

    #[serde(rename = "f")]
    #[serde(deserialize_with = "deserialize_ldtk_flags")]
    pub flip: BVec2,
}

pub struct IntGridTile {
    pub position: IVec2,
    pub value: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityInstance {
    #[serde(rename = "__grid")]
    #[serde(deserialize_with = "deserialize_ldtk_point")]
    pub grid: IVec2,

    pub field_instances: Vec<FieldInstance>,
}

impl EntityInstance {
    pub fn get_field_instance(&self, id: &str) -> Option<&FieldInstance> {
        self.field_instances.iter().find(|x| x.identifier == id)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInstance {
    #[serde(rename = "__identifier")]
    pub identifier: String,

    // TODO: Make this nicer
    pub value: Option<serde_json::Value>,
}

fn deserialize_ldtk_point<'de, D>(de: D) -> Result<IVec2, D::Error>
where
    D: Deserializer<'de>,
{
    let vals: [i32; 2] = Deserialize::deserialize(de)?;

    Ok(IVec2::from(vals))
}

fn deserialize_ldtk_flags<'de, D>(de: D) -> Result<BVec2, D::Error>
where
    D: Deserializer<'de>,
{
    let flags: u8 = Deserialize::deserialize(de)?;

    let x = flags == 1 || flags == 3;
    let y = flags == 2 || flags == 3;

    Ok(BVec2::new(x, y))
}
