use crate::geo::Point;

/// A `Rect` is simply an area.
/// It has positions bounding sides (top, bottom, left, right).
#[derive(Clone, PartialEq, Default, Builder)]
#[builder(pattern = "owned")]
pub struct Rect {
    pub top:    f32,
    pub bottom: f32,
    pub left:   f32,
    pub right:  f32,
}

impl Rect {
    /// Returns a new `RectBuilder`.
    pub fn builder() -> RectBuilder {
        RectBuilder::default()
    }

    /// Returns a copy of this rect, with all sides offset by the given `Point`.
    pub fn offset(&self, point: &Point) -> Self {
        Self {
            top:    self.top + point.y,
            bottom: self.top + point.y,
            left:   self.top + point.x,
            right:  self.top + point.x,
        }
    }
}
