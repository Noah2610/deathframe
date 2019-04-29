use super::component_prelude::*;

/// For entities which have `Velocity` and `MaxVelocity`, their velocities will never
/// be larger than the positive values of `MaxVelocity`, and will never be
/// smaller than the negative values of `MaxVelocity`.
/// Fields are both optional; if a field is `None`, then there is no cap for that value.
#[derive(Serialize, Deserialize)]
pub struct MaxVelocity {
    pub x:                     Option<f32>,
    pub y:                     Option<f32>,
    pub(crate) should_limit_x: bool,
    pub(crate) should_limit_y: bool,
}

impl MaxVelocity {
    /// Create a new `MaxVelocity` component with both `x` and `y` fields.
    pub fn with_xy(x: f32, y: f32) -> Self {
        Self {
            x:              Some(x),
            y:              Some(y),
            should_limit_x: true,
            should_limit_y: true,
        }
    }

    /// Create a new `MaxVelocity` component with only a max `x` value.
    pub fn with_x(x: f32) -> Self {
        Self {
            x:              Some(x),
            y:              None,
            should_limit_x: true,
            should_limit_y: true,
        }
    }

    /// Create a new `MaxVelocity` component with only a max `y` value.
    pub fn with_y(y: f32) -> Self {
        Self {
            x:              None,
            y:              Some(y),
            should_limit_x: true,
            should_limit_y: true,
        }
    }

    /// Skip velocity limitation next frame.
    pub fn dont_limit(&mut self) {
        self.dont_limit_x();
        self.dont_limit_y();
    }

    /// Skip velocity limitation (x) next frame.
    pub fn dont_limit_x(&mut self) {
        self.should_limit_x = true;
    }

    /// Skip velocity limitation (y) next frame.
    pub fn dont_limit_y(&mut self) {
        self.should_limit_y = true;
    }
}

impl Component for MaxVelocity {
    type Storage = VecStorage<Self>;
}

impl From<(f32, f32)> for MaxVelocity {
    fn from(data: (f32, f32)) -> Self {
        Self::with_xy(data.0, data.1)
    }
}

impl From<Vector<f32>> for MaxVelocity {
    fn from(data: Vector<f32>) -> Self {
        Self::with_xy(data.0, data.1)
    }
}

impl From<(Option<f32>, Option<f32>)> for MaxVelocity {
    fn from(data: (Option<f32>, Option<f32>)) -> Self {
        Self {
            x:              data.0,
            y:              data.1,
            should_limit_x: true,
            should_limit_y: true,
        }
    }
}
