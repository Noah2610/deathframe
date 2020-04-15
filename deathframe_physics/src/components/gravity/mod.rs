#[cfg(test)]
mod tests;

use super::component_prelude::*;

/// Entities with the `Gravity` component are affected by gravity.
/// The gravity's strength is applied to the entity's velocity every frame
/// through the `ApplyGravitySystem`.
#[derive(Clone, Debug, Default, Component, Builder, Deserialize)]
#[storage(VecStorage)]
#[builder(pattern = "owned", setter(strip_option), default)]
pub struct Gravity {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

impl Gravity {
    /// Creates a new `GravityBuilder`.
    pub fn builder() -> GravityBuilder {
        GravityBuilder::default()
    }

    /// Set the gravity strength for the given `Axis` to the given value.
    pub fn set(&mut self, axis: &Axis, strength: f32) {
        match axis {
            Axis::X => self.x = Some(strength),
            Axis::Y => self.y = Some(strength),
        }
    }

    /// Returns the gravity strength of the given `Axis`.
    pub fn get(&self, axis: &Axis) -> Option<f32> {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }
}

impl From<(Option<f32>, Option<f32>)> for Gravity {
    fn from(gravs: (Option<f32>, Option<f32>)) -> Self {
        Self {
            x: gravs.0,
            y: gravs.1,
        }
    }
}
