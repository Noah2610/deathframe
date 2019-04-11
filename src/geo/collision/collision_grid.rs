use amethyst::ecs::world::Index;

use super::super::Vector;
use super::CollisionRect;

/// A collection of `CollisionRect`, can perform collision detection.
pub struct CollisionGrid<T> {
    pub rects: Vec<CollisionRect<T>>,
}

impl<T> CollisionGrid<T> {
    /// Create a new `CollisionGrid` by passing in a vector of `CollisionRect`s.
    pub fn new(rects: Vec<CollisionRect<T>>) -> Self {
        Self { rects }
    }

    /// Get a stored `CollisionRect` by its entity ID.
    pub fn rect_by_id(&self, id: Index) -> Option<&CollisionRect<T>> {
        self.rects.iter().find(|rect| id == rect.id)
    }

    /// Returns `true` if the passed `CollisionRect` is colliding with any other
    /// `CollisionRect` stored in this `CollisionGrid`.
    pub fn collides_any(&self, target_rect: &CollisionRect<T>) -> bool {
        self.rects
            .iter()
            .any(|rect| Self::do_rects_collide(&target_rect, rect))
    }

    /// Returns a vector of all `CollisionRect`s, that are in collision
    /// with the passed `CollisionRect` (which may or may not exist in this `CollisionGrid`).
    pub fn colliding_with(
        &self,
        target_rect: &CollisionRect<T>,
    ) -> Vec<&CollisionRect<T>> {
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
    pub fn colliding_with_id(&self, id: Index) -> Vec<&CollisionRect<T>> {
        if let Some(target_rect) = self.rect_by_id(id) {
            self.colliding_with(target_rect)
        } else {
            Vec::new()
        }
    }

    /// Returns `true` if the two passed `CollisionRect`s are in collision
    /// (also checks, that their entity IDs are not the same).
    /// TODO: Maybe make this a standalone function, not associated with the `CollisionGrid` struct?
    #[rustfmt::skip]
    pub fn do_rects_collide<U, V>(
        rect_one: &CollisionRect<U>,
        rect_two: &CollisionRect<V>,
    ) -> bool {
        rect_one.id != rect_two.id && (
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

impl<T> From<Vec<(Index, Vector, Option<Vector>)>> for CollisionGrid<T> {
    fn from(data: Vec<(Index, Vector, Option<Vector>)>) -> Self {
        Self::new(
            data.iter()
                .map(|&rect_data| CollisionRect::from(rect_data))
                .collect(),
        )
    }
}

impl<T> From<Vec<(Index, Vector, Option<Vector>, Option<T>)>>
    for CollisionGrid<T>
where
    T: Clone + Copy,
{
    fn from(data: Vec<(Index, Vector, Option<Vector>, Option<T>)>) -> Self {
        Self::new(
            data.iter()
                .map(|&rect_data| CollisionRect::from(rect_data))
                .collect(),
        )
    }
}
