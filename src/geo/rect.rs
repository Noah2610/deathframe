use super::Vector;

/// A `Rect` is simply an area.
/// It has positions bounding sides (top, bottom, left, right).
#[derive(Clone, PartialEq, Default)]
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
}

/// The builder struct for `Rect`.
#[derive(Default)]
pub struct RectBuilder {
    top:    f32,
    bottom: f32,
    left:   f32,
    right:  f32,
}

impl RectBuilder {
    /// Set the top value.
    pub fn top(mut self, top: f32) -> Self {
        self.top = top;
        self
    }

    /// Set the bottom value.
    pub fn bottom(mut self, bottom: f32) -> Self {
        self.bottom = bottom;
        self
    }

    /// Set the left value.
    pub fn left(mut self, left: f32) -> Self {
        self.left = left;
        self
    }

    /// Set the right value.
    pub fn right(mut self, right: f32) -> Self {
        self.right = right;
        self
    }

    /// Infere the fields by the given position.
    pub fn with_pos(mut self, pos: Vector) -> Self {
        self.top = pos.1;
        self.bottom = pos.1;
        self.left = pos.0;
        self.right = pos.0;
        self
    }

    /// Infere the fields by the given position and size;
    /// the position is the _center_ of the rect.
    pub fn with_pos_and_size(mut self, pos: Vector, size: Vector) -> Self {
        self.top = pos.1 + size.1 * 0.5;
        self.bottom = pos.1 - size.1 * 0.5;
        self.left = pos.0 - size.0 * 0.5;
        self.right = pos.0 + size.0 * 0.5;
        self
    }

    /// Infere the fields by the given position and _optional_ size.
    pub fn with_pos_and_maybe_size(
        mut self,
        pos: Vector,
        size_opt: Option<Vector>,
    ) -> Self {
        self = if let Some(size) = size_opt {
            self.with_pos_and_size(pos, size)
        } else {
            self.with_pos(pos)
        };
        self
    }

    /// Create a `Rect` from this builder.
    pub fn build(self) -> Rect {
        let RectBuilder {
            top,
            bottom,
            left,
            right,
        } = self;
        Rect {
            top,
            bottom,
            left,
            right,
        }
    }
}
