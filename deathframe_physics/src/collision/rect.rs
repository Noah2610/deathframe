use crate::collision::tag::CollisionTag;
use amethyst::ecs::world::Index;
use core::amethyst;
use core::geo::prelude::*;

/// A collision hitbox with a unique entity ID.
/// Holds multiple `Rect`s as the hitbox.
/// Can also hold optional custom data.
#[derive(Clone, Debug)]
pub struct CollisionRect<C, T>
where
    C: CollisionTag,
{
    pub id:     Index,
    pub rects:  Vec<Rect>,
    pub tag:    C,
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
    rects:  Vec<Rect>,
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
            rects:  Vec::new(),
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

    /// Set the `rects`.
    pub fn rects(mut self, rects: Vec<Rect>) -> Self {
        self.rects = rects;
        self
    }

    /// Add a `Rect` to the rects vec.
    pub fn rect(mut self, rect: Rect) -> Self {
        self.rects.push(rect);
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
    pub fn build(self) -> Result<CollisionRect<C, T>, amethyst::Error> {
        let CollisionRectBuilder {
            id,
            rects,
            tag,
            custom,
        } = self;
        Ok(CollisionRect {
            id: id.ok_or_else(|| {
                amethyst::Error::from_string(
                    "CollisionRectBuilder requires an id",
                )
            })?,
            rects,
            tag: tag.ok_or_else(|| {
                amethyst::Error::from_string(
                    "CollisionRectBuilder requires a tag",
                )
            })?,
            custom,
        })
    }
}
