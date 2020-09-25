use super::collision_check;
use super::rect::CollisionRect;
use crate::collision::tag::CollisionTag;
use std::collections::hash_map::HashMap;
use std::hash::Hash;

/// A collection of `CollisionRect`, can perform collision detection.
#[derive(Debug)]
pub struct CollisionGrid<K, C, T>
where
    K: PartialEq + Eq + Hash,
    C: CollisionTag,
{
    pub rects: HashMap<K, CollisionRect<C, T>>,
}

impl<K, C, T> CollisionGrid<K, C, T>
where
    K: PartialEq + Eq + Hash,
    C: CollisionTag,
{
    /// Create a new `CollisionGrid` with the given hashmap.
    pub fn new(rects: HashMap<K, CollisionRect<C, T>>) -> Self {
        Self { rects }
    }

    /// Returns a new `CollisionGrid` with no `CollisionRect`s.
    pub fn empty() -> Self {
        Self {
            rects: Default::default(),
        }
    }

    pub fn insert(&mut self, key: K, rect: CollisionRect<C, T>) {
        self.rects.insert(key, rect);
    }

    /// Clears all `CollisionRect`s from the `rects` field.
    pub fn clear(&mut self) {
        self.rects.clear();
    }

    pub fn get(&self, key: &K) -> Option<&CollisionRect<C, T>> {
        self.rects.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut CollisionRect<C, T>> {
        self.rects.get_mut(key)
    }

    /// Returns `true` if the passed `CollisionRect` is colliding with any other
    /// `CollisionRect` stored in this `CollisionGrid`.
    pub fn collides_any(&self, target_rect: &CollisionRect<C, T>) -> bool {
        self.rects
            .values()
            .any(|rect| collision_check::do_rects_collide(&target_rect, rect))
    }

    /// Returns a vector of all `CollisionRect`s, that are in collision
    /// with the passed `CollisionRect` (which may or may not exist in this `CollisionGrid`).
    pub fn colliding_with(
        &self,
        target_rect: &CollisionRect<C, T>,
    ) -> Vec<&CollisionRect<C, T>> {
        self.rects
            .values()
            .filter(|rect| {
                collision_check::do_rects_collide(&target_rect, rect)
            })
            .collect()
    }
}

impl<K, C, T> Default for CollisionGrid<K, C, T>
where
    K: PartialEq + Eq + Hash,
    C: CollisionTag,
{
    fn default() -> Self {
        Self {
            rects: Default::default(),
        }
    }
}
