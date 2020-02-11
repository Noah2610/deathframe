use super::component_prelude::*;
use core::geo::Axis;

#[cfg(test)]
mod tests;

#[derive(Component, Default, Builder, Debug)]
#[storage(VecStorage)]
#[builder(pattern = "owned")]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    /// Creates a new `Velocity` with the given initial `x` and `y` velocities.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Set the velocity for the given `Axis` to the given value.
    pub fn set(&mut self, axis: &Axis, vel: f32) {
        match axis {
            Axis::X => self.x = vel,
            Axis::Y => self.y = vel,
        }
    }

    /// Get the current velocity for the given `Axis`.
    pub fn get(&self, axis: &Axis) -> f32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }

    /// Increase the velocity for the given `Axis`, by the given increment.
    pub fn increase(&mut self, axis: &Axis, incr: f32) {
        match axis {
            Axis::X => self.x += incr,
            Axis::Y => self.y += incr,
        }
    }

    /// Increase the velocity for the given `Axis`, by the given increment,
    /// but the velocity can not go above (or below the negative of) the given `max` velocity.
    pub fn increase_with_max(&mut self, axis: &Axis, incr: f32, max: f32) {
        match axis {
            Axis::X => self.x = (self.x + incr).min(max).max(-max),
            Axis::Y => self.y = (self.y + incr).min(max).max(-max),
        }
    }

    /// Clear the velocity of the given axis.
    pub fn clear(&mut self, axis: &Axis) {
        match axis {
            Axis::X => self.x = 0.0,
            Axis::Y => self.y = 0.0,
        }
    }

    /// Clear both velocity axes.
    pub fn clear_all(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}
