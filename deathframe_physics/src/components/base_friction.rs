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
    pub(crate) enabled:    (bool, bool),
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
            enabled: Default::default(),
        }
    }

    /// Set the enabled state for the given `Axis` of this `BaseFriction`.
    pub fn set_enabled(&mut self, axis: &Axis, enabled: bool) {
        match axis {
            Axis::X => self.enabled.0 = enabled,
            Axis::Y => self.enabled.1 = enabled,
        }
    }

    /// Returns the optional friction for the given `Axis`,
    /// but __ONLY if it is enabled for that axis!__
    pub(crate) fn get(&self, axis: &Axis) -> Option<f32> {
        if self.is_enabled(axis) {
            self.friction_x
        } else {
            None
        }
    }

    /// Returns `true` if the friction for the given `Axis` is enabled.
    fn is_enabled(&self, axis: &Axis) -> bool {
        match axis {
            Axis::X => self.enabled.0,
            Axis::Y => self.enabled.1,
        }
    }
}

impl Default for BaseFriction {
    fn default() -> Self {
        Self {
            friction_x: None,
            friction_y: None,
            enabled:    (true, true),
        }
    }
}
