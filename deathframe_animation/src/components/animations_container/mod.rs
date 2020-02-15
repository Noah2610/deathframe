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
    K: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    animations:
        HashMap<K, Box<dyn Fn() -> Box<dyn AnimationFramesIter> + Send + Sync>>,
    current_animation: Option<K>,
}

impl<K> AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Debug + Clone,
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

    /// Returns the _key_ of the currently active animation, if any.
    pub fn current(&self) -> Option<&K> {
        self.current_animation.as_ref()
    }

    /// Returns a new `Animation` associated to the given _key_, if any.
    pub fn animation(&self, key: &K) -> Option<Animation> {
        self.animations.get(key).map(|func| func().into())
    }

    /// Returns a new `Animation`, for the current animation _key_, if any.
    pub fn current_animation(&self) -> Option<Animation> {
        self.current().and_then(|key| self.animation(key))
    }
}
