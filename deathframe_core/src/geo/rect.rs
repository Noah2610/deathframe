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

    /// Offsets all sides by the given `Point`.
    pub fn with_offset(mut self, point: &Point) -> Self {
        self.top += point.y;
        self.bottom += point.y;
        self.left += point.x;
        self.right += point.x;
        self
    }

    /// Adds a padding to all sides.
    /// Subtracts/adds the `Point`'s x value from/to the `left`/`right` field,
    /// and the y value from/to the `bottom`/`top` field (respectively).
    pub fn with_padding(mut self, padding: &Point) -> Self {
        self.top += padding.y;
        self.bottom -= padding.y;
        self.left -= padding.x;
        self.right += padding.x;
        self
    }
}
