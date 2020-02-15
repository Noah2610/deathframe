use crate::components::component_prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct AnimationsContainerBuilder<K>
where
    K: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    animations:
        HashMap<K, Box<dyn Fn() -> Box<dyn AnimationFramesIter> + Send + Sync>>,
    current_animation: Option<K>,
}

impl<K> AnimationsContainerBuilder<K>
where
    K: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    /// Add an `Animation` associated to a _key_ to the `AnimationsContainer`.
    /// You add an animation, by giving this method a `Fn`, which _returns_
    /// a new `AnimationFramesIter` (`Box`ed).
    pub fn with<F>(mut self, key: K, animation_gen: F) -> Self
    where
        F: 'static + Fn() -> Box<dyn AnimationFramesIter> + Send + Sync,
    {
        self.animations.insert(key, Box::new(animation_gen));
        self
    }

    /// Set the initally current animation.
    /// An animation with the given _key_ must have been added previously.
    pub fn current(mut self, key: K) -> Result<Self, String> {
        if !self.animations.contains_key(&key) {
            return Err(String::from(
                "Given key to AnimationsContainerBuilder::current doesn't \
                 exist as animation",
            ));
        }
        self.current_animation = Some(key);
        Ok(self)
    }

    /// Build the `AnimationsContainer`.
    pub fn build(self) -> Result<AnimationsContainer<K>, ()> {
        Ok(AnimationsContainer {
            animations:        self.animations,
            current_animation: self.current_animation,
        })
    }
}

impl<K> Default for AnimationsContainerBuilder<K>
where
    K: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    fn default() -> Self {
        Self {
            animations:        HashMap::new(),
            current_animation: None,
        }
    }
}
