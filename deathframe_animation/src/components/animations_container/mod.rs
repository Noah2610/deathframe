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
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
#[cfg_attr(
    feature = "deserialize",
    serde(from = "HashMap<K, AnimationTypeWrapper<Vec<(usize, u64)>>>")
)]
pub struct AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Clone + Debug,
{
    animations:      HashMap<K, Animation>,
    #[cfg_attr(feature = "deserialize", serde(skip))]
    animation_stack: Vec<K>,
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
    /// Plays the animation at the lowest part of the animation stack,
    /// so any other animations, pushed ontop of the stack need
    /// to finish playing, before getting to this animation.
    pub fn play(&mut self, key: K) -> Result<(), String> {
        if self.animations.contains_key(&key) {
            if let Some(base_animation_key) = self.animation_stack.get_mut(0) {
                *base_animation_key = key;
            } else {
                self.animation_stack.push(key);
            }
            Ok(())
        } else {
            Err(String::from(format!(
                "Animation with the key {:?} doesn't exist",
                key
            )))
        }
    }

    /// Push an animation on top of the animation stack,
    /// making this animation play before others.
    /// Animations lower in the stack will continue playing once
    /// upper ones finish or are popped off.
    /// Returns an Error if no animation with the given key exists.
    pub fn push(&mut self, key: K) -> Result<(), String> {
        if self.animations.contains_key(&key) {
            self.animation_stack.push(key);
            Ok(())
        } else {
            Err(String::from(format!(
                "Animation with the key {:?} doesn't exist",
                key
            )))
        }
    }

    /// Pop off an animation from the top of the animation stack,
    /// letting lower ones continue playing.
    /// Returns the popped-off animation key.
    /// Returns an Error if no animation with the given key exists,
    /// or if attempted to pop off when no animation is in the stack.
    /// Note, that it is possible to pop off _all_ animations from the stack,
    /// which may lead to unexpected behaviour.
    pub fn pop(&mut self, key: K) -> Result<K, String> {
        if self.animations.contains_key(&key) {
            self.animation_stack.pop().ok_or(String::from(
                "Attempted to pop off animation from animation stack with no \
                 more animations",
            ))
        } else {
            Err(String::from(format!(
                "Animation with the key {:?} doesn't exist",
                key
            )))
        }
    }

    /// Returns the _key_ of the currently active animation, if any.
    pub fn current(&self) -> Option<&K> {
        self.animation_stack.last()
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
            animations:      animations
                .into_iter()
                .map(|(k, a)| (k, a.into()))
                .collect(),
            animation_stack: Default::default(),
        }
    }
}
