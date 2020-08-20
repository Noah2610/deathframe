use super::component_prelude::*;

/// Friction that is applied constantly.
/// You can disable it with the `set_enabled` function.
#[derive(Component, Builder, Clone, Debug, Deserialize)]
#[storage(VecStorage)]
#[builder(pattern = "owned", setter(strip_option), default)]
pub struct BaseFriction {
    #[serde(alias = "x")]
    pub(crate) friction_x: Option<f32>,
    #[serde(alias = "y")]
    pub(crate) friction_y: Option<f32>,
    #[builder(setter(skip))]
    #[serde(default = "default_enabled")]
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
            enabled: (true, true),
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
        match axis {
            Axis::X if self.enabled.0 => self.friction_x,
            Axis::Y if self.enabled.1 => self.friction_y,
            _ => None,
        }
    }
}

impl BaseFrictionBuilder {
    /// Set the friction value for the given `Axis`.
    pub fn friction(mut self, axis: &Axis, friction: f32) -> Self {
        match axis {
            Axis::X => self.friction_x = Some(Some(friction)),
            Axis::Y => self.friction_y = Some(Some(friction)),
        }
        self
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

impl From<(Option<f32>, Option<f32>)> for BaseFriction {
    fn from(fricts: (Option<f32>, Option<f32>)) -> Self {
        Self::new(fricts.0, fricts.1)
    }
}

fn default_enabled() -> (bool, bool) {
    (true, true)
}
