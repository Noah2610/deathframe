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
#[derive(Component, Default, Clone)]
#[storage(DenseVecStorage)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Clone + Debug,
{
    animations:        HashMap<K, Animation>,
    current_animation: Option<K>,
}

impl<K> AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Clone + Debug,
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
        self.animations.get(key).cloned()
    }

    /// Returns a new `Animation`, for the current animation _key_, if any.
    pub fn current_animation(&self) -> Option<Animation> {
        self.current().and_then(|key| self.animation(key))
    }
}

impl<K, A> From<HashMap<K, A>> for AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Clone + Debug,
    A: Into<Animation>,
{
    fn from(animations: HashMap<K, A>) -> Self {
        Self {
            animations:        animations
                .into_iter()
                .map(|(k, a)| (k, a.into()))
                .collect(),
            current_animation: Default::default(),
        }
    }
}
