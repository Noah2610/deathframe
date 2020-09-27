use super::component_prelude::*;

/// With this component, moving entities only move by
/// whole integer numbers. Velocity float remainder
/// is ignored when moving.
/// Use this for entities that should try to snap
/// to the nearest integer position.
#[derive(Component, Default, Deserialize, Clone)]
#[storage(NullStorage)]
#[serde(deny_unknown_fields)]
pub struct NonPreciseMovement;
