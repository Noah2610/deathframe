use amethyst::ecs::world::Index;

use super::super::Vector;

/// A rectangular collision area with a unique entity ID.
/// Can also hold optional custom data.
#[derive(Clone)]
pub struct CollisionRect<T> {
    pub id: Index,
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
    /// Optional, custom data.
    pub custom: Option<T>,
}

impl<T> CollisionRect<T> {
    /// Create a new `CollisionRect` _without_ custom data.
    /// the passed position `Vector` should be the _center_ of the entity.
    pub fn new(id: Index, position: Vector, size_opt: Option<Vector>) -> Self {
        Self::with_custom(id, position, size_opt, None)
    }

    /// Create a new `CollisionRect` _with_ custom data (still optional).
    /// the passed position `Vector` should be the _center_ of the entity.
    pub fn with_custom(
        id: Index,
        position: Vector,
        size_opt: Option<Vector>,
        custom: Option<T>,
    ) -> Self {
        if let Some(size) = size_opt {
            CollisionRect {
                id:     id,
                top:    position.1 + size.1 * 0.5,
                bottom: position.1 - size.1 * 0.5,
                left:   position.0 - size.0 * 0.5,
                right:  position.0 + size.0 * 0.5,
                custom: custom,
            }
        } else {
            CollisionRect {
                id:     id,
                top:    position.1,
                bottom: position.1,
                left:   position.0,
                right:  position.0,
                custom: custom,
            }
        }
    }
}

impl<T> From<(Index, Vector, Option<Vector>)> for CollisionRect<T> {
    fn from((id, pos, size): (Index, Vector, Option<Vector>)) -> Self {
        Self::new(id, pos, size)
    }
}

impl<T> From<(Index, Vector, Option<Vector>, Option<T>)> for CollisionRect<T> {
    fn from(
        (id, pos, size, custom): (Index, Vector, Option<Vector>, Option<T>),
    ) -> Self {
        Self::with_custom(id, pos, size, custom)
    }
}
