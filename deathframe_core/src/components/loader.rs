use super::component_prelude::*;

/// `Loader` entities can load `Loadable` entities,
/// whose transforms are within a loading distance
/// from the `Loader` entity's transform.
#[derive(Component)]
#[storage(VecStorage)]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Loader {
    pub(crate) loading_distance: (f32, f32),
}

impl Loader {
    /// Create a new `Loader` with the given `x` and `y` loading distances.
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            loading_distance: (x, y),
        }
    }
}

impl From<(f32, f32)> for Loader {
    fn from(loading_distance: (f32, f32)) -> Self {
        Self { loading_distance }
    }
}
