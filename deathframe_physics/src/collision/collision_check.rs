//! Module containing collision checking functions.

use super::prelude::*;
use super::tag::CollisionTag;
use core::amethyst::ecs::world::Index;
use core::geo::prelude::*;

/// Returns `true` if the two passed `CollisionRect`s are in collision;
/// also checks, that their entity IDs are not the same,
/// and that their tags allow them to collide with each other.
pub fn do_rects_collide<C, U, V>(
    rect_one: &CollisionRect<C, U>,
    rect_two: &CollisionRect<C, V>,
) -> bool
where
    C: CollisionTag,
{
    !do_rect_ids_match(&rect_one.id, &rect_two.id)
        && do_rect_tags_match(&rect_one.tag, &rect_two.tag)
        && do_rects_intersect(&rect_one.rect, &rect_two.rect)
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
pub fn do_rect_tags_match<C>(
    tag_one_opt: &Option<C>,
    tag_two_opt: &Option<C>,
) -> bool
where
    C: CollisionTag,
{
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
