use gdnative::api::*;
use gdnative::prelude::*;
use serde::Deserialize;

use super::shape::Shape;
use crate::puzzles::TileDefinition;
use crate::util;

pub const TILE_SIDE_LEN: f32 = 100f32;
pub const TILE_SIZE: Vector2 = Vector2 {
    x: TILE_SIDE_LEN,
    y: TILE_SIDE_LEN,
};

const BORDER_SIZE: f32 = 2f32;
pub const TILE_INNER_SIDE_LEN: f32 = TILE_SIDE_LEN - BORDER_SIZE * 2.0;
pub const TILE_INNER_OFFSET: Vector2 = Vector2 {
    x: BORDER_SIZE,
    y: BORDER_SIZE,
};

const TILE_BACKGROUND_COLOR: Color = Color {
    r: 0.2f32,
    g: 0.2f32,
    b: 0.2f32,
    a: 1.0f32,
};

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum TileType {
    Foreground,
    Background,
}

impl TileType {
    pub fn from_interactable(interactable: bool) -> Self {
        if interactable {
            TileType::Foreground
        } else {
            TileType::Background
        }
    }

    fn to_foreground_color(&self) -> Color {
        match self {
            TileType::Foreground => Color {
                r: 0.5f32,
                g: 0.5f32,
                b: 0.5f32,
                a: 1.0f32,
            },
            TileType::Background => Color {
                r: 0.3f32,
                g: 0.3f32,
                b: 0.3f32,
                a: 1.0f32,
            },
        }
    }
}

#[derive(NativeClass, Debug)]
#[inherit(Area2D)]
#[no_constructor]
pub struct Tile {
    pos: Vector2,
    tile_type: TileType,
}

#[methods]
impl Tile {
    #[method]
    fn _ready(&self, #[base] base: &Area2D) {
        base.set_position(self.pos * TILE_SIZE);
    }

    #[method]
    fn _input_event(
        &self,
        #[base] base: &Area2D,
        _viewport: Ref<Object>,
        raw_event: Ref<InputEvent>,
        _shape_idx: i32,
    ) {
        if let Some(event) = raw_event.clone().cast::<InputEventMouseButton>() {
            if let TileType::Background = self.tile_type {
                return;
            }

            let event = unsafe { event.assume_safe() };

            let parent = unsafe {
                base.get_parent()
                    .expect("Tile does not have a parent")
                    .assume_safe()
                    .cast::<Node2D>()
                    .unwrap()
            };

            let parent_instance = parent
                .cast_instance::<Shape>()
                .expect("Tile's parent is not a shape");

            parent_instance
                .map_mut(|p, _owner| p.update_dragged(parent.as_ref(), event.as_ref()))
                .expect("Failed to set is_being_dragged")
        }
    }
}

impl Tile {
    pub fn from_definition(
        definition: &TileDefinition,
        base_type: TileType,
    ) -> Instance<Self, Unique> {
        let tile_type = definition.tile_type.unwrap_or(base_type);

        let instance = Self {
            pos: definition.pos,
            tile_type,
        }
        .emplace();

        let bg = util::create_square(TILE_SIDE_LEN, TILE_BACKGROUND_COLOR);
        instance.base().add_child(bg, false);

        let fg =
            util::create_square(TILE_INNER_SIDE_LEN, tile_type.to_foreground_color()).into_shared();
        instance.base().add_child(fg, false);
        unsafe { fg.assume_safe() }.set_position(TILE_INNER_OFFSET);

        let collision_shape = CollisionShape2D::new();
        let rectangle = RectangleShape2D::new();
        rectangle.set_extents(TILE_SIZE);
        collision_shape.set_shape(rectangle);
        instance.base().add_child(collision_shape, false);

        instance
    }
}
