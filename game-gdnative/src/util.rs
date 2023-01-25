use gdnative::api::*;
use gdnative::prelude::*;

pub fn screen_center(node: &Node) -> Vector2 {
    unsafe { node.get_viewport().unwrap().assume_safe().size() / 2.0 }
}

pub fn create_square(side_len: f32, color: Color) -> Ref<Polygon2D, Unique> {
    let square = Polygon2D::new();
    square.set_polygon(PoolArray::from_slice(&[
        Vector2::new(0.0, 0.0),
        Vector2::new(side_len, 0.0),
        Vector2::new(side_len, side_len),
        Vector2::new(0.0, side_len),
    ]));
    square.set_color(color);

    square
}
