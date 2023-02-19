use std::collections::HashMap;
use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Position(pub usize, pub usize);

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Position) -> Self {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Position) -> Self {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Position {
    fn min(p1: Position, p2: Position) -> Position {
        Position(usize::min(p1.0, p2.0), usize::min(p1.1, p2.1))
    }

    fn max(p1: Position, p2: Position) -> Position {
        Position(usize::max(p1.0, p2.0), usize::max(p1.1, p2.1))
    }
}

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

fn tiles_from_raw_positions(raw_positions: &[Position]) -> Vec<TileDefinition> {
    let tl_pos = raw_positions.iter().copied().reduce(Position::min).unwrap();

    raw_positions
        .iter()
        .map(|raw_pos| *raw_pos - tl_pos)
        .map(|pos| TileDefinition {
            pos,
            tile_type: None,
        })
        .collect()
}

impl PuzzleDefinition {
    pub fn from_ascii_art(name: String, art: String) -> Self {
        let mut shapes: HashMap<u8, Vec<Position>> = HashMap::new();
        let mut raw_background_positions = vec![];
        for (y, line) in art.lines().enumerate() {
            for (x, chr) in line.as_bytes().iter().enumerate() {
                if char::from(*chr).is_whitespace() {
                    continue;
                }

                if !shapes.contains_key(chr) {
                    shapes.insert(*chr, vec![]);
                }

                shapes.get_mut(chr).unwrap().push(Position(x, y));
                raw_background_positions.push(Position(x, y));
            }
        }

        let shapes = [ShapeDefinition {
            tiles: Shape::RawTiles(tiles_from_raw_positions(&raw_background_positions)),
            pos: None,
            interactable: false,
        }]
        .into_iter()
        .chain(
            shapes
                .into_iter()
                .map(|(_, raw_positions)| ShapeDefinition {
                    tiles: Shape::RawTiles(tiles_from_raw_positions(&raw_positions)),
                    pos: None,
                    interactable: true,
                }),
        )
        .collect();

        PuzzleDefinition { name, shapes }
    }
}
