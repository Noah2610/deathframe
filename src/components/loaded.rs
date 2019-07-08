//! Entities which have `Loadable` and `Loaded` will be included in collision detection.

use super::component_prelude::*;

#[derive(Default)]
pub struct Loaded;

impl Component for Loaded {
    type Storage = NullStorage<Self>;
}
