use gdnative::prelude::*;

use super::shape::{Shape, GLOBAL_GRID_SNAP};

use crate::{puzzles::PuzzleDefinition, tile::TILE_SIDE_LEN};
include!(concat!(env!("OUT_DIR"), "/puzzle_definitions.rs"));

#[derive(NativeClass)]
#[inherit(Node2D)]
#[no_constructor]
pub struct Puzzle {
    shapes: Vec<Instance<Shape>>,
}

#[methods]
impl Puzzle {}

impl Puzzle {
    pub fn from_idx(idx: usize) -> Instance<Self, Unique> {
        let puzzle = serde_yaml::from_str::<PuzzleDefinition>(&PUZZLES[idx].1)
            .expect("Failed to parse puzzle");

        let instance = Self {
            shapes: puzzle
                .shapes
                .iter()
                .scan(GLOBAL_GRID_SNAP, |tl_pos, shape_def| {
                    let (shape, height) = Shape::from_definition(*tl_pos, shape_def);

                    if shape_def.pos.is_none() {
                        *tl_pos += Vector2::new(0f32, height.y * TILE_SIDE_LEN * 1.1);
                    }

                    Some(shape)
                })
                .map(Instance::into_shared)
                .collect(),
        }
        .emplace();

        // Attach shapes
        instance
            .map(|puzzle, node| {
                puzzle
                    .shapes
                    .iter()
                    .for_each(|shape| node.add_child(shape.base(), true))
            })
            .unwrap();

        instance
    }

    pub fn validate(&self, _base: TRef<Node2D>) -> bool {
        self.shapes.iter().all(|shape| {
            unsafe { shape.assume_safe() }
                .map(|shape, shape_node| shape.validate(shape_node, &self.shapes))
                .unwrap_or(false)
        })
    }
}
