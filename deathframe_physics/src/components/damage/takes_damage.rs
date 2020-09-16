use super::component_prelude::*;

/// Makes the entity _take damage_ from entities it
/// collides with, that have the `DealsDamage` component.
#[derive(Component, Default, Clone, Deserialize)]
#[storage(NullStorage)]
#[serde(deny_unknown_fields)]
pub struct TakesDamage;
