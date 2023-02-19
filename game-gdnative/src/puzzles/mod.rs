use serde::{Deserialize, Serialize};

use crate::puzzle::Puzzle;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Position(pub usize, pub usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct PuzzleDefinition {
    pub name: String,
    pub shapes: Vec<ShapeDefinition>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Shape {
    RawTiles(Vec<TileDefinition>),
    Rect(usize, usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapeDefinition {
    #[serde(default)]
    pub pos: Option<Position>,
    #[serde(default = "bool_true")]
    pub interactable: bool,

    tiles: Shape,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileType {
    Foreground,
    Background,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TileDefinition {
    pub pos: Position,
    #[serde(default)]
    pub tile_type: Option<TileType>,
}

fn bool_true() -> bool {
    true
}

impl ShapeDefinition {
    pub fn get_tiles(&self) -> Vec<TileDefinition> {
        match self.tiles.clone() {
            Shape::RawTiles(tiles) => tiles,
            Shape::Rect(width, height) => itertools::iproduct!(0..width, 0..height)
                .map(|(x, y)| TileDefinition {
                    pos: Position(x, y),
                    tile_type: None,
                })
                .collect(),
        }
    }
}

impl PuzzleDefinition {
    pub fn from_ascii_art(name: String, art: String) -> Self {
        todo!()
    }
}
