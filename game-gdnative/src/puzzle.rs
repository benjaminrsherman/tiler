use gdnative::prelude::*;

use super::shape::Shape;

use crate::puzzles::PuzzleDefinition;
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
    pub fn from_idx(idx: i64) -> Instance<Self, Unique> {
        let puzzle = serde_yaml::from_str::<PuzzleDefinition>(&PUZZLES[idx as u64 as usize].1)
            .expect("Failed to parse puzzle");

        let instance = Self {
            shapes: puzzle
                .shapes
                .iter()
                .map(Shape::from_definition)
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

    pub fn validate(&self, base: TRef<Node2D>) -> bool {
        self.shapes.iter().all(|shape| {
            unsafe { shape.assume_safe() }
                .map(|shape, shape_node| shape.validate(shape_node, &self.shapes))
                .unwrap_or(false)
        })
    }
}
