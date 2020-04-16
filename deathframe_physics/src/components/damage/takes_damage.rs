use super::component_prelude::*;

/// Makes the entity _take damage_ from entities it
/// collides with, that have the `DealsDamage` component.
#[derive(Component, Default, Clone, Deserialize)]
#[storage(NullStorage)]
pub struct TakesDamage;
