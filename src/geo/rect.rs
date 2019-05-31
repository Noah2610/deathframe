/// A `Rect` is simply an area.
/// It has positions bounding sides (top, bottom, left, right).
#[derive(Clone, PartialEq)]
pub struct Rect {
    pub top:    f32,
    pub bottom: f32,
    pub left:   f32,
    pub right:  f32,
}
