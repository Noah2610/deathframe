use super::component_prelude::*;

/// Entities which have `Unloaded` are ignored in many systems.
#[derive(Default, Component, Deserialize, Clone)]
#[storage(NullStorage)]
#[serde(deny_unknown_fields)]
pub struct Unloaded;
