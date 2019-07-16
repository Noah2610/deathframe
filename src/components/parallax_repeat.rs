use super::component_prelude::*;

/// Additional component for `Parallax` entities, whose textures should be repeated
/// horizontally/vertically.
#[derive(Default)]
pub struct ParallaxRepeat {
    pub repeat_x: bool,
    pub repeat_y: bool,
}

impl ParallaxRepeat {
    /// Creates a new `ParallaxRepeat` component with the the given
    /// `repeat_x` and `repeat_y` booleans as arguments.
    pub fn new(repeat_x: bool, repeat_y: bool) -> Self {
        Self { repeat_x, repeat_y }
    }
}

impl Component for ParallaxRepeat {
    type Storage = VecStorage<Self>;
}
