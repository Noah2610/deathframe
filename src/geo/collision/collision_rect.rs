use amethyst::ecs::world::Index;

use super::super::Rect;
use super::super::Vector;
use crate::components::solid::SolidTag;

/// A rectangular collision area with a unique entity ID.
/// Can also hold optional custom data.
#[derive(Clone)]
pub struct CollisionRect<STag, T>
where
    STag: SolidTag,
{
    pub id: Index,
    pub rect: Rect,
    /// Solid tag
    pub tag: Option<STag>,
    /// Optional, custom data.
    pub custom: Option<T>,
}

impl<STag, T> CollisionRect<STag, T>
where
    STag: SolidTag,
{
    /// Create a new `CollisionRect` _without_ custom data and with the default `STag` (solid tag).
    /// The passed position `Vector` should be the _center_ of the entity.
    pub fn new(id: Index, position: Vector, size_opt: Option<Vector>) -> Self {
        Self::with_custom(id, position, size_opt, None, None)
    }

    /// Create a new `CollisionRect` _with_ custom data (still optional).
    /// The passed position `Vector` should be the _center_ of the entity.
    pub fn with_custom(
        id: Index,
        position: Vector,
        size_opt: Option<Vector>,
        tag: Option<STag>,
        custom: Option<T>,
    ) -> Self {
        if let Some(size) = size_opt {
            CollisionRect {
                id:     id,
                rect:   Rect {
                    top:    position.1 + size.1 * 0.5,
                    bottom: position.1 - size.1 * 0.5,
                    left:   position.0 - size.0 * 0.5,
                    right:  position.0 + size.0 * 0.5,
                },
                tag:    tag,
                custom: custom,
            }
        } else {
            CollisionRect {
                id:     id,
                rect:   Rect {
                    top:    position.1,
                    bottom: position.1,
                    left:   position.0,
                    right:  position.0,
                },
                tag:    tag,
                custom: custom,
            }
        }
    }
}

impl<STag, T> From<(Index, Vector, Option<Vector>)> for CollisionRect<STag, T>
where
    STag: SolidTag,
{
    fn from((id, pos, size): (Index, Vector, Option<Vector>)) -> Self {
        Self::new(id, pos, size)
    }
}

impl<STag, T> From<(Index, Vector, Option<Vector>, STag, Option<T>)>
    for CollisionRect<STag, T>
where
    STag: SolidTag,
{
    fn from(
        (id, pos, size, tag, custom): (
            Index,
            Vector,
            Option<Vector>,
            STag,
            Option<T>,
        ),
    ) -> Self {
        Self::with_custom(id, pos, size, Some(tag), custom)
    }
}
