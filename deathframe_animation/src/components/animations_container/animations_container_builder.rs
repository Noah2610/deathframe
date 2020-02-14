use crate::components::prelude::{Animation, AnimationsContainer};
use std::collections::HashMap;
use std::hash::Hash;

pub struct AnimationsContainerBuilder<K>
where
    K: 'static + Hash + Eq + Send + Sync,
{
    animations: HashMap<K, Animation>,
}

impl<K> Default for AnimationsContainerBuilder<K>
where
    K: 'static + Hash + Eq + Send + Sync,
{
    fn default() -> Self {
        Self {
            animations: HashMap::new(),
        }
    }
}

impl<K> AnimationsContainerBuilder<K>
where
    K: 'static + Hash + Eq + Send + Sync,
{
    pub fn build(self) -> Result<AnimationsContainer<K>, ()> {
        Ok(AnimationsContainer {
            animations:        self.animations,
            current_animation: None,
        })
    }
}
