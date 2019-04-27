//! TODO: Documentation

use std::collections::HashMap;

use super::component_prelude::*;
use super::Animation;

#[derive(Default)]
pub struct AnimationsContainer {
    animations:  HashMap<String, Animation>,
    pub current: Option<Animation>,
}

impl AnimationsContainer {
    pub fn new() -> AnimationsContainerBuilder {
        AnimationsContainerBuilder::default()
    }

    pub fn get<T>(&self, name: T) -> Option<Animation>
    where
        T: ToString,
    {
        self.animations.get(&name.to_string()).map(Clone::clone)
    }

    pub fn set<T>(&mut self, name: T)
    where
        T: ToString,
    {
        let name = name.to_string();
        self.current = Some(
            self.animations
                .get(&name)
                .expect(&format!(
                    "Animation does not exist for AnimationsContainer: {}",
                    name
                ))
                .clone(),
        );
    }
}

impl Component for AnimationsContainer {
    type Storage = DenseVecStorage<Self>;
}

pub struct AnimationsContainerBuilder {
    animations: HashMap<String, Animation>,
    current:    Option<Animation>,
}

impl AnimationsContainerBuilder {
    pub fn insert<T>(mut self, name: T, animation: Animation) -> Self
    where
        T: ToString,
    {
        self.animations.insert(name.to_string(), animation);
        self
    }

    pub fn current<T>(mut self, name: T) -> Self
    where
        T: ToString,
    {
        self.current = Some(
            self.animations
                .get(&name.to_string())
                .expect(&format!(
                    "Animation does not exist for AnimationsContainerBuilder: \
                     {}",
                    name.to_string()
                ))
                .clone(),
        );
        self
    }

    pub fn build(self) -> AnimationsContainer {
        AnimationsContainer {
            animations: self.animations,
            current:    self.current,
        }
    }
}

impl Default for AnimationsContainerBuilder {
    fn default() -> Self {
        Self {
            animations: HashMap::new(),
            current:    None,
        }
    }
}
