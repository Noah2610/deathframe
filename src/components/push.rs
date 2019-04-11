use super::component_prelude::*;

/// `Push` can push other `Pushable` entities,
/// when moving (with `Transform` and `Velocity`).
/// NOTE: The whole pushing system is very janky, best not to use it.
///       Or if you do use it, I recommend making the `Push` entity static,
///       ie. _not_ giving it a `Velocity`.
pub struct Push;

impl Component for Push {
    type Storage = NullStorage<Self>;
}

impl Default for Push {
    fn default() -> Self {
        Self
    }
}
