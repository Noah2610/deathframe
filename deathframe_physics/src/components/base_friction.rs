use super::component_prelude::*;

/// Friction that is applied constantly.
/// You can disable it with the `set_enabled` function.
#[derive(Component)]
#[storage(VecStorage)]
pub struct BaseFriction {
    pub(crate) friction: f32,
    pub(crate) enabled:  bool,
}

impl BaseFriction {
    /// Create a new `BaseFriction` component with the given friction value.
    pub fn new(friction: f32) -> Self {
        Self {
            friction,
            enabled: true,
        }
    }

    /// Set the enabled state of this `BaseFriction`.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
