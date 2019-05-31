use crate::geo::Rect;

use super::component_prelude::*;

/// Entities' `Transform`s are _confined_ to an area with this component.
/// Their `Transform` may never leave this confined area.
pub struct Confined {
    pub rect: Rect,
}

impl Component for Confined {
    type Storage = VecStorage<Self>;
}

impl From<Rect> for Confined {
    fn from(rect: Rect) -> Self {
        Self { rect }
    }
}

impl From<&Rect> for Confined {
    fn from(rect: &Rect) -> Self {
        Self { rect: rect.clone() }
    }
}
