/// Just a plain `Axis` enum with `X` and `Y` variants.
#[derive(Clone, Debug, PartialEq)]
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

    /// Returns `true` if this is the `X` variant.
    pub fn is_x(&self) -> bool {
        Axis::X == *self
    }

    /// Returns `true` if this is the `Y` variant.
    pub fn is_y(&self) -> bool {
        Axis::Y == *self
    }
}
