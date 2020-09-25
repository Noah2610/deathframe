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
    pub rects: HashMap<K, Vec<CollisionRect<C, T>>>,
}

impl<K, C, T> CollisionGrid<K, C, T>
where
    K: PartialEq + Eq + Hash,
    C: CollisionTag,
{
    /// Create a new `CollisionGrid` by passing in a vector of `CollisionRect`s.
    pub fn new(rects: HashMap<K, Vec<CollisionRect<C, T>>>) -> Self {
        Self { rects }
    }

    /// Returns a new `CollisionGrid` with no `CollisionRect`s.
    pub fn empty() -> Self {
        Self {
            rects: Default::default(),
        }
    }

    pub fn insert(&mut self, key: K, rects: Vec<CollisionRect<C, T>>) {
        self.rects.insert(key, rects);
    }

    pub fn append(&mut self, key: K, mut rects: Vec<CollisionRect<C, T>>) {
        self.rects.entry(key).or_default().append(&mut rects);
    }

    /// Appends the given `Vec<CollisionRect>` to the grid.
    /// Probably more efficient than using `push` in a loop.
    /// Note that we take ownership of the passed Vec,
    /// instead of copying `Vec::append`'s signature and only taking
    /// a mutable reference; I prefer it this way, as this can avoids problems,
    /// when you pass a mutable reference but try to use the Vec again afterwards (with no items).
    pub fn extend(&mut self, rects: HashMap<K, Vec<CollisionRect<C, T>>>) {
        self.rects.extend(rects);
    }

    /// Clears all `CollisionRect`s from the `rects` field.
    pub fn clear(&mut self) {
        self.rects.clear();
    }

    pub fn get(&self, key: &K) -> Option<&Vec<CollisionRect<C, T>>> {
        self.rects.get(key)
    }

    pub fn get_mut(
        &mut self,
        key: &K,
    ) -> Option<&mut Vec<CollisionRect<C, T>>> {
        self.rects.get_mut(key)
    }

    /// Returns `true` if the passed `CollisionRect` is colliding with any other
    /// `CollisionRect` stored in this `CollisionGrid`.
    pub fn collides_any(&self, target_rect: &CollisionRect<C, T>) -> bool {
        self.rects.values().any(|rects| {
            rects.iter().any(|rect| {
                collision_check::do_rects_collide(&target_rect, rect)
            })
        })
    }

    /// Returns a vector of all `CollisionRect`s, that are in collision
    /// with the passed `CollisionRect` (which may or may not exist in this `CollisionGrid`).
    pub fn colliding_with(
        &self,
        target_rect: &CollisionRect<C, T>,
    ) -> Vec<&CollisionRect<C, T>> {
        let mut colliding_rects = Vec::new();
        for rects in self.rects.values() {
            rects
                .iter()
                .filter(|rect| {
                    collision_check::do_rects_collide(&target_rect, rect)
                })
                .for_each(|colliding_rect| {
                    colliding_rects.push(colliding_rect)
                });
        }
        colliding_rects
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
