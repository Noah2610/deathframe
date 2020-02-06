/// Miscellaneous `Side` enum. Used somewhere related to collision detection.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Side {
    // `Inner` isn't actually a side, but it makes life easier having this here
    Inner,
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    /// Returns `true` when side is `Inner`.
    pub fn is_inner(&self) -> bool {
        self == &Side::Inner
    }

    /// Returns `true` when side is `Top`.
    pub fn is_top(&self) -> bool {
        self == &Side::Top
    }

    /// Returns `true` when side is `Bottom`.
    pub fn is_bottom(&self) -> bool {
        self == &Side::Bottom
    }

    /// Returns `true` when side is `Left`.
    pub fn is_left(&self) -> bool {
        self == &Side::Left
    }

    /// Returns `true` when side is `Right`.
    pub fn is_right(&self) -> bool {
        self == &Side::Right
    }
}
