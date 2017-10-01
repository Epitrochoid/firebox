use na::{Point2, Vector2};

pub struct BBox {
    tag: String,
    center: Point2<f32>,
    halfwidth: Vector2<f32>,
    halfheight: Vector2<f32>
}
