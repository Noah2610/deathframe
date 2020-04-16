use super::component_prelude::*;

/// _Deals damage_ to entities that collide with this entity,
/// that have the `TakesDamage` component.
#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct DealsDamage {
    pub damage: HitPoints,
}
