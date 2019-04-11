use super::component_prelude::*;

/// Entities, which have `Velocity` are moved by their velocity, every frame.
#[derive(Debug, Deserialize)]
pub struct Velocity {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
