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
        let puzzle = serde_yaml::from_str::<PuzzleDefinition>(&PUZZLES[idx])
            .expect("Failed to parse puzzle");

        let shape_colors = colorgrad::warm().colors(puzzle.shapes.len());

        let instance = Self {
            shapes: puzzle
                .shapes
                .iter()
                .zip(shape_colors.iter())
                .scan(GLOBAL_GRID_SNAP, |tl_pos, (shape_def, raw_color)| {
                    let shape_color = Color {
                        r: raw_color.r as f32,
                        b: raw_color.b as f32,
                        g: raw_color.g as f32,
                        a: raw_color.a as f32,
                    };

                    let (shape, height) = Shape::from_definition(*tl_pos, shape_def, shape_color);

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
