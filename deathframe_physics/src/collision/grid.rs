use specs::world::Index;

use super::rect::CollisionRect;
use crate::collision::tag::CollisionTag;
use core::geo::prelude::*;

/// A collection of `CollisionRect`, can perform collision detection.
pub struct CollisionGrid<C, T>
where
    C: CollisionTag,
{
    pub rects: Vec<CollisionRect<C, T>>,
}

impl<C, T> CollisionGrid<C, T>
where
    C: CollisionTag,
{
    /// Create a new `CollisionGrid` by passing in a vector of `CollisionRect`s.
    pub fn new(rects: Vec<CollisionRect<C, T>>) -> Self {
        Self { rects }
    }

    /// Returns a new `CollisionGrid` with no `CollisionRect`s.
    pub fn empty() -> Self {
        Self { rects: Vec::new() }
    }

    /// Adds a new `CollisionRect` to the grid.
    pub fn push(&mut self, rect: CollisionRect<C, T>) {
        self.rects.push(rect);
    }

    /// Clears all `CollisionRect`s from the `rects` field.
    pub fn clear(&mut self) {
        self.rects.clear();
    }

    /// Get a stored `CollisionRect` by its entity ID.
    pub fn rect_by_id(&self, id: Index) -> Option<&CollisionRect<C, T>> {
        self.rects.iter().find(|rect| {
            if let Some(other_id) = rect.id {
                id == other_id
            } else {
                false
            }
        })
    }

    /// Returns `true` if the passed `CollisionRect` is colliding with any other
    /// `CollisionRect` stored in this `CollisionGrid`.
    pub fn collides_any(&self, target_rect: &CollisionRect<C, T>) -> bool {
        self.rects
            .iter()
            .any(|rect| Self::do_rects_collide(&target_rect, rect))
    }

    /// Returns a vector of all `CollisionRect`s, that are in collision
    /// with the passed `CollisionRect` (which may or may not exist in this `CollisionGrid`).
    pub fn colliding_with(
        &self,
        target_rect: &CollisionRect<C, T>,
    ) -> Vec<&CollisionRect<C, T>> {
        self.rects
            .iter()
            .filter(|rect| Self::do_rects_collide(&target_rect, rect))
            .collect()
    }

    /// Similar to the `colliding_with` method, but instead of passing
    /// a `CollisionRect` (which may or may not exist in this `CollisionGrid`),
    /// you pass in an entity ID to a `CollisionRect` which is stored inside this `CollisionGrid`.
    /// Note that, if you pass in an ID, which does not exist as a `CollisionRect` in this
    /// `CollisionGrid`, then you will simply receive an empty vector.
    pub fn colliding_with_id(&self, id: Index) -> Vec<&CollisionRect<C, T>> {
        if let Some(target_rect) = self.rect_by_id(id) {
            self.colliding_with(target_rect)
        } else {
            Vec::new()
        }
    }

    /// Returns `true` if the two passed `CollisionRect`s are in collision;
    /// also checks, that their entity IDs are not the same,
    /// and that their tags allow them to collide with each other.
    /// TODO: Maybe make this a standalone function, not associated with the `CollisionGrid` struct?
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

impl<C, T> Default for CollisionGrid<C, T>
where
    C: CollisionTag,
{
    fn default() -> Self {
        Self { rects: Vec::new() }
    }
}
