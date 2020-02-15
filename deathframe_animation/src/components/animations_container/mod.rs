mod animations_container_builder;
#[cfg(test)]
mod tests;

use super::component_prelude::*;
use animations_container_builder::AnimationsContainerBuilder;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// A component, which can hold multiple `Animation`s.
/// It knows which `Animation` is currently active / playing.
/// The `SwitchAnimationsSystem` will switch out the entity's
/// `Animation` component with the active animation from this component.
#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Debug,
{
    animations:        HashMap<K, Animation>,
    current_animation: Option<K>,
}

impl<K> AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Debug,
{
    /// Returns an `AnimationsContainerBuilder`
    pub fn builder() -> AnimationsContainerBuilder<K> {
        AnimationsContainerBuilder::default()
    }

    /// Play the animation with the given _key_.
    /// Returns an Error if an animation with the given key doesn't exist.
    pub fn play(&mut self, key: K) -> Result<(), String> {
        if self.animations.contains_key(&key) {
            self.current_animation = Some(key);
            Ok(())
        } else {
            Err(String::from(format!(
                "Animation with the key {:?} doesn't exist",
                key
            )))
        }
    }
}
