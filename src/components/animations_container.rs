//! TODO: Documentation

use std::collections::HashMap;
use std::time::Instant;

use super::component_prelude::*;
use super::Animation;

#[derive(Default)]
pub struct AnimationsContainer {
    animations:    HashMap<String, Animation>,
    pub current:   Option<(String, Animation)>,
    pub play_once: Option<(String, Animation)>,
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
        if !self.is_current(&name) {
            let mut animation = self
                .animations
                .get(&name)
                .expect(&format!(
                    "Animation does not exist for AnimationsContainer: {}",
                    name
                ))
                .clone();
            animation.switch_now = true;
            self.current = Some((name.clone(), animation));
        }
    }

    pub fn unset(&mut self) {
        self.current = None;
    }

    /// Play once
    pub fn play<T>(&mut self, name: T)
    where
        T: ToString,
    {
        let name = name.to_string();
        if !self.is_play_once(&name) {
            let mut animation = self
                .animations
                .get(&name)
                .expect(&format!(
                    "Animation does not exist for AnimationsContainer: {}",
                    name
                ))
                .clone();
            animation.switch_now = true;
            self.play_once = Some((name.clone(), animation));
        }
    }

    pub fn is_current<T>(&self, target_name: T) -> bool
    where
        T: ToString,
    {
        if let Some((name, _)) = &self.current {
            &target_name.to_string() == name
        } else {
            false
        }
    }

    pub fn is_play_once<T>(&self, target_name: T) -> bool
    where
        T: ToString,
    {
        if let Some((name, _)) = &self.play_once {
            &target_name.to_string() == name
        } else {
            false
        }
    }
}

impl Component for AnimationsContainer {
    type Storage = DenseVecStorage<Self>;
}

pub struct AnimationsContainerBuilder {
    animations: HashMap<String, Animation>,
    current:    Option<(String, Animation)>,
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
        let name = name.to_string();
        self.current = Some((
            name.clone(),
            self.animations
                .get(&name)
                .expect(&format!(
                    "Animation does not exist for AnimationsContainerBuilder: \
                     {}",
                    name
                ))
                .clone(),
        ));
        self
    }

    pub fn build(self) -> AnimationsContainer {
        AnimationsContainer {
            animations: self.animations,
            current:    self.current,
            play_once:  None,
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
