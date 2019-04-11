use super::component_prelude::*;

/// Entities, which have `Velocity` and `Gravity`, will be affected by gravitational pull.
/// This means, their velocity is increased by the gravity's force (`x`, `y`) every frame.
#[derive(Debug, Deserialize)]
pub struct Gravity {
    pub x: f32,
    pub y: f32,
}

impl Gravity {
    /// Create a new `Gravity` component with the given `x` and `y` values.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for Gravity {
    type Storage = VecStorage<Self>;
}

impl From<(f32, f32)> for Gravity {
    fn from(data: (f32, f32)) -> Self {
        Self::new(data.0, data.1)
    }
}
