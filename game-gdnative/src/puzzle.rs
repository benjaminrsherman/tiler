use gdnative::prelude::*;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::shape::{Shape, GLOBAL_GRID_SNAP};

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
        let puzzle = serde_yaml::from_str::<PuzzleDefinition>(&PUZZLES[idx]).unwrap();

        let mut shape_colors = colorgrad::warm().colors(puzzle.shapes.len());
        shape_colors.shuffle(&mut ChaCha8Rng::seed_from_u64(puzzle.shapes.len() as u64));

        let instance = Self {
            shapes: puzzle
                .shapes
                .iter()
                .zip(shape_colors.iter())
                .scan(
                    (GLOBAL_GRID_SNAP, 0f32),
                    |(tl_pos, max_col_width), (shape_def, raw_color)| {
                        let shape_color = Color {
                            r: raw_color.r as f32,
                            b: raw_color.b as f32,
                            g: raw_color.g as f32,
                            a: raw_color.a as f32,
                        };

                        let shape = if shape_def.interactable {
                            let (shape, shape_size) =
                                Shape::from_definition(*tl_pos, shape_def, shape_color);

                            if shape_def.pos.is_none() {
                                *tl_pos += Vector2::new(
                                    0f32,
                                    shape_size.y * TILE_SIDE_LEN + GLOBAL_GRID_SNAP.y,
                                );

                                *max_col_width =
                                    f32::max(*max_col_width, shape_size.x * TILE_SIDE_LEN);
                            }

                            shape
                        } else {
                            Shape::from_definition(GLOBAL_GRID_SNAP, shape_def, shape_color).0
                        };

                        // If we're outside of the window, reset to the next column
                        if tl_pos.y
                            >= gdnative::api::OS::godot_singleton().window_size().y
                                - GLOBAL_GRID_SNAP.y * 2.0
                        {
                            *tl_pos = Vector2::new(
                                tl_pos.x + *max_col_width + GLOBAL_GRID_SNAP.x,
                                GLOBAL_GRID_SNAP.y,
                            );
                            *max_col_width = 0f32;
                        }

                        Some(shape)
                    },
                )
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
