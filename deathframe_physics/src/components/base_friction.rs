use super::component_prelude::*;

/// Friction that is applied constantly.
/// You can disable it with the `set_enabled` function.
#[derive(Component, Builder)]
#[storage(VecStorage)]
#[builder(pattern = "owned", setter(strip_option), default)]
pub struct BaseFriction {
    pub(crate) friction_x: Option<f32>,
    pub(crate) friction_y: Option<f32>,
    #[builder(setter(skip))]
    pub(crate) enabled:    bool,
}

impl BaseFriction {
    /// Create a new `BaseFrictionBuilder`
    pub fn builder() -> BaseFrictionBuilder {
        BaseFrictionBuilder::default()
    }

    /// Create a new `BaseFriction` component with the given optional
    /// friction values, for the `x` and `y` axes, respectively.
    pub fn new(friction_x: Option<f32>, friction_y: Option<f32>) -> Self {
        Self {
            friction_x,
            friction_y,
            enabled: true,
        }
    }

    /// Set the enabled state of this `BaseFriction`.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl Default for BaseFriction {
    fn default() -> Self {
        Self {
            friction_x: None,
            friction_y: None,
            enabled:    true,
        }
    }
}
