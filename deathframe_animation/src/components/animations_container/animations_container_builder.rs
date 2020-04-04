use crate::components::component_prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub struct AnimationsContainerBuilder<K>
where
    K: 'static + Hash + Eq + Send + Sync + Clone + Debug,
{
    animations:        HashMap<K, Animation>,
    current_animation: Option<K>,
}

impl<K> AnimationsContainerBuilder<K>
where
    K: 'static + Hash + Eq + Send + Sync + Clone + Debug,
{
    /// Add an `Animation` associated to a _key_ to the `AnimationsContainer`.
    pub fn with(mut self, key: K, anim: Animation) -> Self {
        self.animations.insert(key, anim);
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
            animations:      self.animations,
            animation_stack: self
                .current_animation
                .map(|current| vec![current])
                .unwrap_or_else(Default::default),
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
