use specs::world::Index;

use crate::collision::tag::CollisionTag;
use core::geo::prelude::*;

fn default_tag<C: CollisionTag>() -> Option<C> {
    None
}

fn default_custom<T>() -> Option<T> {
    None
}

/// A rectangular collision area with a unique entity ID.
/// Can also hold optional custom data.
#[derive(Clone)]
pub struct CollisionRect<C, T>
where
    C: CollisionTag,
{
    pub id:     Option<Index>,
    pub rect:   Rect,
    pub tag:    Option<C>,
    pub custom: Option<T>,
}

impl<C, T> CollisionRect<C, T>
where
    C: CollisionTag,
{
    /// Returns a new `CollisionRectBuilder`.
    pub fn builder() -> CollisionRectBuilder<C, T> {
        CollisionRectBuilder::default()
    }
}

/// Builder struct for `CollisionRect`.
pub struct CollisionRectBuilder<C, T>
where
    C: CollisionTag,
{
    id:     Option<Index>,
    rect:   Rect,
    tag:    Option<C>,
    custom: Option<T>,
}

impl<C, T> Default for CollisionRectBuilder<C, T>
where
    C: CollisionTag,
{
    fn default() -> Self {
        Self {
            id:     None,
            rect:   Default::default(),
            tag:    None,
            custom: None,
        }
    }
}

impl<C, T> CollisionRectBuilder<C, T>
where
    C: CollisionTag,
{
    /// Set the `id`.
    pub fn id(mut self, id: Index) -> Self {
        self.id = Some(id);
        self
    }

    /// Set the `rect`.
    pub fn rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    /// Set the `tag`.
    pub fn tag(mut self, tag: C) -> Self {
        self.tag = Some(tag);
        self
    }

    /// Set the `custom`.
    pub fn custom(mut self, custom: T) -> Self {
        self.custom = Some(custom);
        self
    }

    /// Set the `custom`, given as an `Option`.
    pub fn custom_maybe(mut self, custom_opt: Option<T>) -> Self {
        self.custom = custom_opt;
        self
    }

    /// Infere the `rect: Rect` field by the given position,
    /// _assuming there is no size_.
    pub fn with_pos(mut self, pos: Vector) -> Self {
        self.rect = Rect {
            top:    pos.y,
            bottom: pos.y,
            left:   pos.x,
            right:  pos.x,
        };
        self
    }

    /// Infere the `rect: Rect` field by the given position and size;
    /// the position is the _center_ of the rect.
    pub fn with_pos_and_size(mut self, pos: Vector, size: Vector) -> Self {
        self.rect = Rect {
            top:    pos.y + size.y * 0.5,
            bottom: pos.y - size.y * 0.5,
            left:   pos.x - size.x * 0.5,
            right:  pos.x + size.x * 0.5,
        };
        self
    }

    /// Infere the `rect: Rect` field by the given position and _optional_ size.
    pub fn with_pos_and_maybe_size(
        mut self,
        pos: Vector,
        size_opt: Option<Vector>,
    ) -> Self {
        self = if let Some(size) = size_opt {
            self.with_pos_and_size(pos, size)
        } else {
            self.with_pos(pos)
        };
        self
    }

    /// Create a `CollisionRect` from this builder.
    pub fn build(self) -> CollisionRect<C, T> {
        let CollisionRectBuilder {
            id,
            rect,
            tag,
            custom,
        } = self;
        CollisionRect {
            id,
            rect,
            tag,
            custom,
        }
    }
}

impl<C, T> From<(Index, Vector, Option<Vector>)> for CollisionRect<C, T>
where
    C: CollisionTag,
{
    fn from((id, pos, size_opt): (Index, Vector, Option<Vector>)) -> Self {
        CollisionRectBuilder::default()
            .id(id)
            .with_pos_and_maybe_size(pos, size_opt)
            .build()
    }
}

impl<C, T> From<(Index, Vector, Option<Vector>, C, Option<T>)>
    for CollisionRect<C, T>
where
    C: CollisionTag,
{
    fn from(
        (id, pos, size_opt, tag, custom_opt): (
            Index,
            Vector,
            Option<Vector>,
            C,
            Option<T>,
        ),
    ) -> Self {
        CollisionRectBuilder::default()
            .id(id)
            .with_pos_and_maybe_size(pos, size_opt)
            .tag(tag)
            .custom_maybe(custom_opt)
            .build()
    }
}
