use gdnative::{api::InputEventMouseButton, prelude::*};

use super::tile::{Tile, TileType, TILE_SIDE_LEN, TILE_SIZE};
use crate::puzzles::ShapeDefinition;
use crate::util::IVector2;

// Godot Derives
#[derive(NativeClass, Debug)]
#[inherit(Node2D)]
#[no_constructor]
// Serde Derives
pub struct Shape {
    drag_pos_start: Option<(Vector2, Vector2)>,

    tiles: Vec<Instance<Tile>>,

    pos: Vector2,
}

const GLOBAL_GRID_SNAP: Vector2 = Vector2 {
    x: TILE_SIDE_LEN / 10.0,
    y: TILE_SIDE_LEN / 10.0,
};

#[methods]
impl Shape {
    #[method]
    fn _ready(&self, #[base] base: &Node2D) {
        base.set_position(self.pos * TILE_SIZE);
    }

    #[method]
    fn _process(&self, #[base] base: &Node2D, _delta: f64) {
        if let Some((self_start_pos, mouse_start_pos)) = self.drag_pos_start {
            let viewport = unsafe { base.get_viewport().unwrap().assume_safe() };

            let mouse_diff = viewport.get_mouse_position() - mouse_start_pos;

            base.set_global_position((self_start_pos + mouse_diff).snapped(GLOBAL_GRID_SNAP));
        }
    }
}

impl Shape {
    pub fn update_dragged(&mut self, base: &Node2D, event: &InputEventMouseButton) {
        if event.is_pressed() {
            self.drag_pos_start = Some((base.global_position(), event.position()))
        } else {
            self.drag_pos_start = None;
            self.pos = base.position();
        }
    }

    pub fn from_definition(definition: &ShapeDefinition) -> Instance<Self, Unique> {
        let instance = Self {
            tiles: definition
                .tiles
                .iter()
                .map(|tile_def| {
                    Tile::from_definition(
                        tile_def,
                        TileType::from_interactable(definition.interactable),
                    )
                })
                .map(Instance::into_shared)
                .collect(),
            drag_pos_start: None,
            pos: definition.pos,
        }
        .emplace();

        // Attach tiles
        instance
            .map(|shape, node| {
                shape
                    .tiles
                    .iter()
                    .for_each(|tile| node.add_child(tile, true))
            })
            .unwrap();

        instance
    }

    pub fn validate(&self, base: TRef<Node2D>, all_shapes: &[Instance<Shape>]) -> bool {
        // TODO: tell the tile which shape it's in

        self.tiles.iter().all(|tile| {
            unsafe { tile.assume_safe() }
                .map(|tile, tile_node| tile.validate(tile_node, &all_shapes))
                .unwrap_or(false)
        })
    }

    /// Returns true if any tile in this shape matches the filter function and overlaps with `tgt_tile`
    pub fn overlaps_with_tile<FilterFunc>(
        &self,
        tgt_tile: TInstance<Tile>,
        filter: FilterFunc,
    ) -> bool
    where
        FilterFunc: FnMut(&TInstance<Tile>) -> bool,
    {
        let tgt_loc = IVector2::from(tgt_tile.base().global_position());

        self.tiles
            .iter()
            .map(|tile_instance| unsafe { tile_instance.assume_safe() })
            .filter(filter)
            .any(|tile| IVector2::from(tile.base().global_position()) == tgt_loc)
    }
}
