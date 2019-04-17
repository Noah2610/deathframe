use super::Vector;

/// Anchor points. Used with some components,
/// where specifying which anchor point is used is necessary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Anchor {
    TopLeft,
    TopRight,
    TopMiddle,
    BottomLeft,
    BottomRight,
    BottomMiddle,
    MiddleLeft,
    MiddleRight,
    Middle,
}

impl Anchor {
    /// Returns the top-left position as a `Vector`.
    #[rustfmt::skip]
    pub fn top_left_for(&self, pos: Vector, size: Vector) -> Vector {
        match self {
            Anchor::TopLeft      => pos,
            Anchor::TopRight     => (pos.0 - size.0, pos.1).into(),
            Anchor::TopMiddle    => (pos.0 - size.0 * 0.5, pos.1).into(),
            Anchor::BottomLeft   => (pos.0, pos.1 + size.1).into(),
            Anchor::BottomRight  => (pos.0 - size.0, pos.1 + size.1).into(),
            Anchor::BottomMiddle => (pos.0 - size.0 * 0.5, pos.1 + size.1).into(),
            Anchor::MiddleLeft   => (pos.0, pos.1 + size.1 * 0.5).into(),
            Anchor::MiddleRight  => (pos.0 - size.0, pos.1 + size.1 * 0.5).into(),
            Anchor::Middle       => (pos.0 - size.0 * 0.5, pos.1 + size.1 * 0.5).into(),
        }
    }

    /// Returns the bottom-left position as a `Vector`.
    pub fn bottom_left_for(&self, pos: Vector, size: Vector) -> Vector {
        let top_left = self.top_left_for(pos, size);
        (top_left.0, top_left.1 - size.1).into()
    }

    /// Returns the middle-left position as a `Vector`.
    pub fn middle_left_for(&self, pos: Vector, size: Vector) -> Vector {
        let top_left = self.top_left_for(pos, size);
        (top_left.0, top_left.1 - size.1 * 0.5).into()
    }

    /// Returns the top-right position as a `Vector`.
    pub fn top_right_for(&self, pos: Vector, size: Vector) -> Vector {
        let top_left = self.top_left_for(pos, size);
        (top_left.0 + size.1, top_left.1).into()
    }

    /// Returns the bottom-right position as a `Vector`.
    pub fn bottom_right_for(&self, pos: Vector, size: Vector) -> Vector {
        let top_left = self.top_left_for(pos, size);
        (top_left.0 + size.1, top_left.1 - size.1).into()
    }

    /// Returns the middle-right position as a `Vector`.
    pub fn middle_right_for(&self, pos: Vector, size: Vector) -> Vector {
        let top_left = self.top_left_for(pos, size);
        (top_left.0 + size.1, top_left.1 - size.1 * 0.5).into()
    }

    /// Returns the top-middle position as a `Vector`.
    pub fn top_middle_for(&self, pos: Vector, size: Vector) -> Vector {
        let top_left = self.top_left_for(pos, size);
        (top_left.0 + size.0 * 0.5, top_left.1).into()
    }

    /// Returns the bottom-middle position as a `Vector`.
    pub fn bottom_middle_for(&self, pos: Vector, size: Vector) -> Vector {
        let top_left = self.top_left_for(pos, size);
        (top_left.0 + size.0 * 0.5, top_left.1 - size.1).into()
    }

    /// Returns the middle position as a `Vector`.
    pub fn middle_for(&self, pos: Vector, size: Vector) -> Vector {
        let top_left = self.top_left_for(pos, size);
        (top_left.0 + size.0 * 0.5, top_left.1 - size.1 * 0.5).into()
    }
}
