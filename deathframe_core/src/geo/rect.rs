use crate::components::prelude::Size;
use crate::geo::Point;

/// A `Rect` is simply an area.
/// It has positions bounding sides (top, bottom, left, right).
#[derive(Clone, PartialEq, Default, Builder, Debug, Deserialize)]
#[builder(pattern = "owned", derive(Clone))]
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

    /// Returns the center position of this rect.
    pub fn center(&self) -> Point {
        let half_length = (
            (self.right - self.left) * 0.5,
            (self.top - self.bottom) * 0.5,
        );
        Point::new(self.left + half_length.0, self.bottom + half_length.1)
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

impl From<&Size> for Rect {
    fn from(size: &Size) -> Self {
        let half_size = (size.w * 0.5, size.h * 0.5);
        Self {
            top:    half_size.1,
            bottom: -half_size.1,
            left:   -half_size.0,
            right:  half_size.0,
        }
    }
}

impl From<&Size> for RectBuilder {
    fn from(size: &Size) -> Self {
        let half_size = (size.w * 0.5, size.h * 0.5);
        Self {
            top:    Some(half_size.1),
            bottom: Some(-half_size.1),
            left:   Some(-half_size.0),
            right:  Some(half_size.0),
        }
    }
}
