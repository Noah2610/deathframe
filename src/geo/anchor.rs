use super::Vector;

/// Anchor points. Used with some components,
/// where specifying which anchor point is used is necessary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Anchor {
    LeftTop,
    LeftBottom,
    LeftMiddle,
    RightTop,
    RightBottom,
    RightMiddle,
    MiddleTop,
    MiddleBottom,
    Middle,
}

impl Anchor {
    /// Returns the left-top position as a `Vector`.
    #[rustfmt::skip]
    pub fn left_top_for(&self, pos: Vector, size: Vector) -> Vector {
        match self {
            Anchor::LeftTop      => pos,
            Anchor::LeftBottom   => (pos.0, pos.1 + size.1).into(),
            Anchor::LeftMiddle   => (pos.0, pos.1 + size.1 * 0.5).into(),
            Anchor::RightTop     => (pos.0 - size.0, pos.1).into(),
            Anchor::RightBottom  => (pos.0 - size.0, pos.1 + size.1).into(),
            Anchor::RightMiddle  => (pos.0 - size.0, pos.1 + size.1 * 0.5).into(),
            Anchor::MiddleTop    => (pos.0 - size.0 * 0.5, pos.1).into(),
            Anchor::MiddleBottom => (pos.0 - size.0 * 0.5, pos.1 + size.1).into(),
            Anchor::Middle       => (pos.0 - size.0 * 0.5, pos.1 + size.1 * 0.5).into(),
        }
    }

    /// Returns the left-bottom position as a `Vector`.
    pub fn left_bottom_for(&self, pos: Vector, size: Vector) -> Vector {
        let left_top = self.left_top_for(pos, size);
        (left_top.0, left_top.1 - size.1).into()
    }

    /// Returns the left-middle position as a `Vector`.
    pub fn left_middle_for(&self, pos: Vector, size: Vector) -> Vector {
        let left_top = self.left_top_for(pos, size);
        (left_top.0, left_top.1 - size.1 * 0.5).into()
    }

    /// Returns the right-top position as a `Vector`.
    pub fn right_top_for(&self, pos: Vector, size: Vector) -> Vector {
        let left_top = self.left_top_for(pos, size);
        (left_top.0 + size.1, left_top.1).into()
    }

    /// Returns the right-bottom position as a `Vector`.
    pub fn right_bottom_for(&self, pos: Vector, size: Vector) -> Vector {
        let left_top = self.left_top_for(pos, size);
        (left_top.0 + size.1, left_top.1 - size.1).into()
    }

    /// Returns the right-middle position as a `Vector`.
    pub fn right_middle_for(&self, pos: Vector, size: Vector) -> Vector {
        let left_top = self.left_top_for(pos, size);
        (left_top.0 + size.1, left_top.1 - size.1 * 0.5).into()
    }

    /// Returns the middle-top position as a `Vector`.
    pub fn middle_top_for(&self, pos: Vector, size: Vector) -> Vector {
        let left_top = self.left_top_for(pos, size);
        (left_top.0 + size.0 * 0.5, left_top.1).into()
    }

    /// Returns the middle-bottom position as a `Vector`.
    pub fn middle_bottom_for(&self, pos: Vector, size: Vector) -> Vector {
        let left_top = self.left_top_for(pos, size);
        (left_top.0 + size.0 * 0.5, left_top.1 - size.1).into()
    }

    /// Returns the middle position as a `Vector`.
    pub fn middle_for(&self, pos: Vector, size: Vector) -> Vector {
        let left_top = self.left_top_for(pos, size);
        (left_top.0 + size.0 * 0.5, left_top.1 - size.1 * 0.5).into()
    }
}
