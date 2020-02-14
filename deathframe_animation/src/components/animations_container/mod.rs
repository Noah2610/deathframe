mod animations_container_builder;
#[cfg(test)]
mod tests;

use super::component_prelude::*;
use animations_container_builder::AnimationsContainerBuilder;
use std::collections::HashMap;
use std::hash::Hash;

/// A component, which can hold multiple `Animation`s.
/// It knows which `Animation` is currently active / playing.
/// The `SwitchAnimationsSystem` will switch out the entity's
/// `Animation` component with the active animation from this component.
#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync,
{
    animations:        HashMap<K, Animation>,
    current_animation: Option<K>,
}

impl<K> AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync,
{
    /// Returns an `AnimationsContainerBuilder`
    pub fn builder() -> AnimationsContainerBuilder<K> {
        AnimationsContainerBuilder::default()
    }
}
