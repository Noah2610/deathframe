use specs::world::Index;

use crate::collision::tag::CollisionTag;
use core::geo::prelude::*;

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
#[derive(Clone)]
pub struct CollisionRectBuilder<C, T>
where
    C: CollisionTag,
{
    id:     Option<Index>,
    rect:   Option<Rect>,
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
            rect:   None,
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
        self.rect = Some(rect);
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
            rect: rect.expect("CollisionRectBuilder requires a Rect"),
            tag,
            custom,
        }
    }
}
