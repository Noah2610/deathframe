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
#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
#[serde(from = "HashMap<K, AnimationTypeWrapper<Vec<(usize, u64)>>>")]
pub struct AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Clone + Debug,
{
    animations:              HashMap<K, Animation>,
    #[serde(skip)]
    animation_stack:         Vec<K>,
    #[serde(skip)]
    last_finished_animation: Option<K>,
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
    ///
    /// Sets the `last_finished_animation` to `None`.
    pub fn play(&mut self, key: K) -> Result<(), String> {
        if self.animations.contains_key(&key) {
            if let Some(base_animation_key) = self.animation_stack.get_mut(0) {
                *base_animation_key = key;
            } else {
                self.animation_stack.push(key);
            }
            self.last_finished_animation = None;
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
    /// Will only push animation if the final animation on the stack
    /// isn't already the same animation.
    /// Animations lower in the stack will continue playing once
    /// upper ones finish or are popped off.
    /// Returns an Error if no animation with the given key exists.
    ///
    /// Sets the `last_finished_animation` to `None`.
    pub fn push(&mut self, key: K) -> Result<(), String> {
        if self.animations.contains_key(&key) {
            if self
                .current()
                .map(|current| current != &key)
                .unwrap_or(true)
            {
                self.animation_stack.push(key);
            }
            self.last_finished_animation = None;
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
    /// Returns the popped off animation key.
    /// Returns an Error if attempted to pop off when no animation is in the stack.
    /// Note, that it is possible to pop off _all_ animations from the stack,
    /// which may lead to unexpected behaviour.
    ///
    /// Also sets the `last_finished_animation`.
    pub fn pop(&mut self) -> Result<K, String> {
        self.animation_stack
            .pop()
            .map(|anim| {
                self.last_finished_animation = Some(anim.clone());
                anim
            })
            .ok_or(String::from(
                "Attempted to pop off animation from animation stack with no \
                 more animations",
            ))
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

    /// Returns `true` if an animation for the given _key_ exists.
    pub fn has_animation(&self, key: &K) -> bool {
        self.animations.contains_key(key)
    }

    /// Truncate the `animation_stack` to the given length.
    /// Note, that this should probably never truncate to `0`.
    pub fn truncate_animation_stack(&mut self, truncate_to: usize) {
        self.animation_stack.truncate(truncate_to);
    }

    /// Returns the last animation that was finished and popped off
    /// the `animation_stack`. Can only be a `Once` animation.
    /// The `last_finished_animation` is set to `None` when another
    /// animation starts playing.
    pub fn last_finished_animation(&self) -> Option<&K> {
        self.last_finished_animation.as_ref()
    }
}

impl<K, A> From<HashMap<K, A>> for AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Clone + Debug,
    A: Into<Animation>,
{
    fn from(animations: HashMap<K, A>) -> Self {
        Self {
            animations:              animations
                .into_iter()
                .map(|(k, a)| (k, a.into()))
                .collect(),
            animation_stack:         Default::default(),
            last_finished_animation: None,
        }
    }
}

impl<K> Default for AnimationsContainer<K>
where
    K: 'static + Hash + Eq + Send + Sync + Clone + Debug,
{
    fn default() -> Self {
        Self {
            animations:              HashMap::new(),
            animation_stack:         Vec::new(),
            last_finished_animation: None,
        }
    }
}
