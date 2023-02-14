use gdnative::prelude::Vector2;
use serde::{Deserialize, Deserializer};

use crate::tile::TileType;

#[derive(Debug, Deserialize)]
pub struct PuzzleDefinition {
    pub shapes: Vec<ShapeDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct ShapeDefinition {
    #[serde(deserialize_with = "deserialize_vector2")]
    pub pos: Vector2,
    #[serde(default = "bool_true")]
    pub interactable: bool,
    pub tiles: Vec<TileDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct TileDefinition {
    #[serde(deserialize_with = "deserialize_vector2")]
    pub pos: Vector2,
    #[serde(default)]
    pub tile_type: Option<TileType>,
}

fn deserialize_vector2<'de, D>(deserializer: D) -> Result<Vector2, D::Error>
where
    D: Deserializer<'de>,
{
    <(f32, f32)>::deserialize(deserializer).map(|(x, y)| Vector2 { x, y })
}

fn bool_true() -> bool {
    true
}
