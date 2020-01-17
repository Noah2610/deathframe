use super::component_prelude::*;

/// Entities, which have `Velocity` are moved by their velocity, every frame.
#[derive(Debug, Serialize, Deserialize)]
pub struct Velocity {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
}

impl Velocity {
    /// Create a new `Velocity` component with the given `x` and `y` starting values.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Set the velocity to `0, 0`.
    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    /// Increase the velocity with a given max velocity (`(Option<f32>, Option<f32>)`).
    /// The velocity will only be increased to a maximum of the given max velocity;
    /// if the velocity was already above the given max, no changes are made
    /// and the previous velocity is kept.
    /// Max velocity should have positive values, it will check negative max values appropriately.
    pub fn increase_with_max(
        &mut self,
        increase: Vector,
        max: (Option<f32>, Option<f32>),
    ) {
        if increase.0 != 0.0 {
            self.increase_x_with_max(increase.0, max.0);
        }
        if increase.1 != 0.0 {
            self.increase_y_with_max(increase.1, max.1);
        }
    }

    /// Increase the velocity of the given axis, with a given max velocity.
    /// Same as `increase_with_max`, but only affects one of the axes.
    pub fn increase_axis_with_max(
        &mut self,
        axis: Axis,
        increase: f32,
        max: Option<f32>,
    ) {
        if increase != 0.0 {
            match axis {
                Axis::X => {
                    self.increase_x_with_max(increase, max);
                }
                Axis::Y => {
                    self.increase_y_with_max(increase, max);
                }
            }
        }
    }

    /// Same as `increase_with_max`, but only affects `x` velocity.
    pub fn increase_x_with_max(&mut self, increase: f32, max_opt: Option<f32>) {
        if let Some(max) = max_opt {
            let max = max.abs();
            if is_at_or_under_max(self.x, max) {
                self.x += increase;
                if !is_at_or_under_max(self.x, max) {
                    self.x = max * self.x.signum();
                }
            }
        } else {
            // Given max is `None`, just increase velocity.
            self.x += increase;
        }
    }

    /// Same as `increase_with_max`, but only affects `y` velocity.
    pub fn increase_y_with_max(&mut self, increase: f32, max_opt: Option<f32>) {
        if let Some(max) = max_opt {
            let max = max.abs();
            if is_at_or_under_max(self.y, max) {
                self.y += increase;
                if !is_at_or_under_max(self.y, max) {
                    self.y = max * self.y.signum();
                }
            }
        } else {
            // Given max is `None`, just increase velocity.
            self.y += increase;
        }
    }
}

/// Returns `true` if a positive `num` is equal to or smaller than a positive `max`
/// and if a negative `num` is equal to or larger than a negative `max`.
fn is_at_or_under_max(num: f32, max: f32) -> bool {
    let max = max.abs();
    num <= max && num >= -max
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
