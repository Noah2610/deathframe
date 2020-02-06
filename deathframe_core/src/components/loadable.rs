use super::component_prelude::*;

/// Entities which have `Loadable` may be loaded or unloaded
/// (get or remove the `Loaded` component) later on.
#[derive(Default)]
pub struct Loadable;

impl Component for Loadable {
    type Storage = NullStorage<Self>;
}