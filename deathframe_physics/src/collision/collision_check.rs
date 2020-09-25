//! Module containing collision checking functions.

use super::prelude::*;
use super::tag::CollisionTag;
use core::amethyst::ecs::world::Index;
use core::geo::prelude::*;

/// Returns `true` if the two passed `CollisionRect`s are in collision;
/// also checks, that their entity IDs are not the same,
/// and that their tags allow them to collide with each other.
#[inline]
pub fn do_rects_collide<C, U, V>(
    rect_one: &CollisionRect<C, U>,
    rect_two: &CollisionRect<C, V>,
) -> bool
where
    C: CollisionTag,
{
    !do_rect_ids_match(rect_one.id, rect_two.id)
        && do_rect_tags_match(&rect_one.tag, &rect_two.tag)
        && do_rects_intersect(&rect_one.rect, &rect_two.rect)
}

/// Checks if the given IDs are the same.
#[inline]
pub fn do_rect_ids_match(id_one: Index, id_two: Index) -> bool {
    id_one == id_two
}

/// Checks if the first given tag should collide with the second given tag.
#[inline]
pub fn do_rect_tags_match<C>(tag_one: &C, tag_two: &C) -> bool
where
    C: CollisionTag,
{
    tag_one.collides_with(tag_two)
}

/// Returns `true` if the two passed `Rect`s intersect with each other.
#[inline]
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
