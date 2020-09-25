use super::component_prelude::*;

/// Entities which have `Unloaded` are ignored in many systems.
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Unloaded;

impl Component for Unloaded {
    type Storage = NullStorage<Self>;
}
