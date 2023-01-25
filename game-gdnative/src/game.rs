use gdnative::{
    api::{AcceptDialog, MenuButton},
    prelude::*,
};

use crate::puzzle::Puzzle;

use super::util;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Main {
    puzzle_node: Option<Ref<Node2D>>,
    alert: Option<Ref<AcceptDialog>>,
}

#[methods]
impl Main {
    #[method]
    fn _ready(&mut self, #[base] base: TRef<Node2D>) {
        // Register the UI layer
        let ui = self.get_ui(base);
        self.register_puzzle_select_callback(base, ui, "_on_puzzle_selected");
        self.register_validate_callback(base, ui, "_on_validate_requested");

        self._on_puzzle_selected(base.as_ref(), 0);

        let alert = AcceptDialog::new();
        alert.set_title("lmao u thought");
        alert.set_text("ELi didn't let me write the code for this before I finished 500 puzzles");
        let alert = alert.into_shared();
        base.add_child(alert, false);
        self.alert = Some(alert);
    }

    #[method]
    fn _on_puzzle_selected(&mut self, #[base] base: &Node2D, puzzle_idx: i64) {
        godot_print!("puzzle selected: {}", puzzle_idx);

        if let Some(puzzle) = self.puzzle_node.take() {
            base.remove_child(puzzle);
        }

        let puzzle = Puzzle::from_idx(puzzle_idx).into_base().into_shared();
        self.puzzle_node = Some(puzzle);

        unsafe {
            puzzle
                .assume_safe()
                .set_global_position(util::screen_center(base.upcast::<Node>()))
        }

        base.add_child(puzzle, false);
    }

    #[method]
    fn _on_validate_requested(&self) {
        unsafe { self.alert.unwrap().assume_safe() }.popup_centered_minsize(Vector2::ZERO);
    }
}

impl Main {
    fn new(_base: &Node2D) -> Self {
        Main {
            puzzle_node: None,
            alert: None,
        }
    }

    fn get_ui(&self, base: TRef<Node2D>) -> TRef<CanvasLayer> {
        unsafe {
            base.get_node("UI")
                .expect("Main does not have a UI layer")
                .assume_safe()
                .cast::<CanvasLayer>()
                .expect("UI is not a CanvasLayer")
        }
    }

    fn register_puzzle_select_callback(
        &self,
        base: TRef<Node2D>,
        ui: TRef<CanvasLayer>,
        callback: &str,
    ) {
        unsafe {
            ui.get_node("LevelSelectButton")
                .expect("UI layer does not have a LevelSelectButton")
                .assume_safe()
                .cast::<MenuButton>()
                .expect("LevelSelectButton is not a MenuButton")
                .get_popup()
                .expect("LevelSelectButton does not have a popup")
                .assume_safe()
        }
        .connect(
            "index_pressed",
            base,
            callback,
            VariantArray::new_shared(),
            0,
        )
        .expect("Failed to connect to index_selected signal on popup menu");
    }

    fn register_validate_callback(
        &self,
        base: TRef<Node2D>,
        ui: TRef<CanvasLayer>,
        callback: &str,
    ) {
        unsafe {
            ui.get_node("ValidatePuzzleButton")
                .expect("UI layer does not have a ValidatePuzzleButton")
                .assume_safe()
                .cast::<Button>()
                .expect("ValidatePuzzleButton is not a Button")
        }
        .connect("pressed", base, callback, VariantArray::new_shared(), 0)
        .expect("Failed to connect to pressed signal on ValidatePuzzleButton");
    }
}
