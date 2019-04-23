use super::component_prelude::*;

/// Solid entities cannot move into each other;
/// collision detection between solid entities is performed
/// while moving them by their velocities.
/// Solid only affects moving entities and solid entities moving into non-moving, solid entities.
#[derive(Serialize, Deserialize)]
pub struct Solid;

impl Component for Solid {
    type Storage = NullStorage<Self>;
}

impl Default for Solid {
    fn default() -> Self {
        Self
    }
}
