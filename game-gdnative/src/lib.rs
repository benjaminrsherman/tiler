use gdnative::prelude::*;

mod puzzles;
pub mod util;

mod game;
mod puzzle;
mod shape;
mod tile;
mod ui;

fn init(handle: InitHandle) {
    handle.add_class::<tile::Tile>();
    handle.add_class::<shape::Shape>();
    handle.add_class::<puzzle::Puzzle>();
    handle.add_class::<ui::UI>();
    handle.add_class::<game::Main>();
}
godot_init!(init);
