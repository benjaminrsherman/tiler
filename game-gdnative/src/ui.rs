use gdnative::{api::MenuButton, prelude::*};

include!(concat!(env!("OUT_DIR"), "/puzzle_definitions.rs"));

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
#[register_with(Self::register)]
pub struct UI;

#[methods]
impl UI {
    #[method]
    fn _ready(&self, #[base] base: TRef<CanvasLayer>) {
        let popup_menu = unsafe {
            base.get_node("LevelSelectButton")
                .expect("UI layer does not have a level select button")
                .assume_safe()
                .cast::<MenuButton>()
                .expect("LevelSelectButton is not a MenuButton")
                .get_popup()
                .expect("LevelSelectButton does not have a popup")
                .assume_safe()
        };

        PUZZLES.iter().for_each(|(name, _)| {
            popup_menu.add_item(name, 0, 0);
        });
    }
}

impl UI {
    fn new(_base: &CanvasLayer) -> Self {
        UI
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder
            .signal("puzzle_selected")
            .with_param("puzzle_idx", VariantType::I64)
            .done();
    }
}