use super::rect::CollisionRect;
use crate::collision::tag::CollisionTag;
use core::geo::prelude::*;
use specs::world::Index;
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
            rects
                .iter()
                .any(|rect| Self::do_rects_collide(&target_rect, rect))
        })
    }

    /// Returns a vector of all `CollisionRect`s, that are in collision
    /// with the passed `CollisionRect` (which may or may not exist in this `CollisionGrid`).
    pub fn colliding_with(
        &self,
        target_rect: &CollisionRect<C, T>,
    ) -> Vec<&CollisionRect<C, T>> {
        self.rects
            .values()
            .map(|rects| {
                rects
                    .iter()
                    .filter(|rect| Self::do_rects_collide(&target_rect, rect))
            })
            .flatten()
            .collect()
    }

    /// Returns `true` if the two passed `CollisionRect`s are in collision;
    /// also checks, that their entity IDs are not the same,
    /// and that their tags allow them to collide with each other.
    pub fn do_rects_collide<U, V>(
        rect_one: &CollisionRect<C, U>,
        rect_two: &CollisionRect<C, V>,
    ) -> bool {
        !Self::do_rect_ids_match(&rect_one.id, &rect_two.id)
            && Self::do_rect_tags_match(&rect_one.tag, &rect_two.tag)
            && Self::do_rects_intersect(&rect_one.rect, &rect_two.rect)
    }

    /// Returns `true` if the two passed `Option<Index>` CollisionRect IDs are equal.
    /// Both arguments need to be `Some` for `true` to be returned;
    /// if any of the arguments is `None`, then `false` is returned.
    pub fn do_rect_ids_match(
        id_one_opt: &Option<Index>,
        id_two_opt: &Option<Index>,
    ) -> bool {
        // TODO: I'm pretty sure that I can replace this logic with a pure equality check.
        if let (Some(id_one), Some(id_two)) = (id_one_opt, id_two_opt) {
            id_one == id_two
        } else {
            false
        }
    }

    /// Returns `true` if the two passed `Option<C>` Solid Tags may collide with each other.
    /// Returns `true` if any of the passed arguments is `None`.
    pub fn do_rect_tags_match(
        tag_one_opt: &Option<C>,
        tag_two_opt: &Option<C>,
    ) -> bool {
        if let (Some(tag_one), Some(tag_two)) = (tag_one_opt, tag_two_opt) {
            tag_one.collides_with(tag_two)
        } else {
            true
        }
    }

    /// Returns `true` if the two passed `Rect`s intersect with each other.
    #[rustfmt::skip]
    pub fn do_rects_intersect(rect_one: &Rect, rect_two: &Rect) -> bool {
        (
            (
                   rect_one.left >= rect_two.left
                && rect_one.left <  rect_two.right
            ) || (
                   rect_one.left  <= rect_two.left
                && rect_one.right >  rect_two.left
            )
        ) && (
            (
                   rect_one.top <= rect_two.top
                && rect_one.top >  rect_two.bottom
            ) || (
                   rect_one.top    >= rect_two.top
                && rect_one.bottom <  rect_two.top
            )
        )
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
