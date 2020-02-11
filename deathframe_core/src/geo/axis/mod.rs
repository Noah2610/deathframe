pub mod prelude {
    pub use super::by_axis::ByAxis;
    pub use super::Axis;
    pub use super::Iter as AxisIter;
}

mod by_axis;

use std::fmt;

/// Just a plain `Axis` enum with `X` and `Y` variants.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    /// This iterator-like method simply executes the passed closure with
    /// the `X` variant and then for the `Y` variant.
    pub fn for_each<C>(mut iterate: C)
    where
        C: FnMut(Self),
    {
        iterate(Axis::X);
        iterate(Axis::Y);
    }

    /// Returns an `Iterator` over both axes.
    pub fn iter() -> Iter {
        Iter::default()
    }

    /// Returns `true` if this is the `X` variant.
    pub fn is_x(&self) -> bool {
        Axis::X == *self
    }

    /// Returns `true` if this is the `Y` variant.
    pub fn is_y(&self) -> bool {
        Axis::Y == *self
    }
}

/// An iterator over both axes.
#[derive(Default)]
pub struct Iter(u8);

impl Iterator for Iter {
    type Item = Axis;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        match self.0 {
            1 => Some(Axis::X),
            2 => Some(Axis::Y),
            _ => None,
        }
    }
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Axis::X => write!(f, "x"),
            Axis::Y => write!(f, "y"),
        }
    }
}
