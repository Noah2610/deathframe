use super::component_prelude::*;

/// Entities, which have `Velocity` and `Gravity`, will be affected by gravitational pull.
/// This means, their velocity is increased by the gravity's force (`x`, `y`) every frame.
#[derive(Debug, Serialize, Deserialize)]
pub struct Gravity {
    pub x:              f32,
    pub y:              f32,
    pub(crate) enabled: bool,
}

impl Gravity {
    /// Create a new `Gravity` component with the given `x` and `y` values.
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            enabled: true,
        }
    }

    /// Enable the gravity, if it was previously disabled via the `disable` method.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the gravity, until re-enabled via the `enable` method.
    pub fn disable(&mut self) {
        self.enabled = false;
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

impl From<Vector<f32>> for Gravity {
    fn from(data: Vector<f32>) -> Self {
        Self::new(data.0, data.1)
    }
}
