use gdnative::{
    api::{AcceptDialog, JavaScript, MenuButton},
    prelude::*,
};

use crate::puzzle::Puzzle;
include!(concat!(env!("OUT_DIR"), "/puzzle_definitions.rs"));

use super::util;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Main {
    puzzle_node: Option<Instance<Puzzle>>,
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

        let raw_puzzle_shortname = JavaScript::godot_singleton().eval(
            "(new URLSearchParams(window.location.search)).get('puzzle')",
            true,
        );

        godot_print!("raw_puzzle_shortname is {:?}", raw_puzzle_shortname);

        let init_puzzle_idx = raw_puzzle_shortname
            .to::<String>()
            .and_then(|shortname| shortname.strip_suffix("/").map(str::to_string))
            .and_then(|shortname| PUZZLE_NAME_MAP.get(&shortname))
            .copied()
            .unwrap_or(0);

        self._on_puzzle_selected(base.as_ref(), init_puzzle_idx);

        let alert = AcceptDialog::new();
        let alert = alert.into_shared();
        base.add_child(alert, false);
        self.alert = Some(alert);
    }

    #[method]
    fn _on_puzzle_selected(&mut self, #[base] base: &Node2D, puzzle_idx: usize) {
        godot_print!("puzzle selected: {}", puzzle_idx);

        if let Some(puzzle) = self.puzzle_node.take() {
            base.remove_child(puzzle);
        }

        let puzzle = Puzzle::from_idx(puzzle_idx).into_shared();
        self.puzzle_node = Some(puzzle.clone());

        unsafe {
            puzzle
                .assume_safe()
                .base()
                .set_global_position(util::screen_center(base.upcast::<Node>()))
        }

        base.add_child(puzzle, false);
    }

    #[method]
    fn _on_validate_requested(&self) {
        let valid = unsafe { self.puzzle_node.as_ref().unwrap().assume_safe() }
            .map(Puzzle::validate)
            .unwrap_or(false);

        let alert = unsafe { self.alert.unwrap().assume_safe() };

        if valid {
            alert.set_title("Congratulations!");
            alert.set_text("Your solution is valid.");
        } else {
            alert.set_title("Uh oh!");
            alert.set_text("There's an issue with your solution :(");
        }

        alert.popup_centered_minsize(Vector2::ZERO);
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
