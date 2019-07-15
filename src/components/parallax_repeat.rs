use super::component_prelude::*;

/// Additional component for `Parallax` entities, whose textures should be repeated
/// horizontally/vertically.
pub struct ParallaxRepeat {
    pub axis: Axis,
}

impl ParallaxRepeat {
    pub fn new(axis: Axis) -> Self {
        Self { axis }
    }
}

impl Component for ParallaxRepeat {
    type Storage = VecStorage<Self>;
}
