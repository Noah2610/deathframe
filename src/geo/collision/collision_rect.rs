use amethyst::ecs::world::Index;

use super::super::{Rect, Vector};
use crate::components::solid::SolidTag;

/// A rectangular collision area with a unique entity ID.
/// Can also hold optional custom data.
#[derive(Clone, Default)]
pub struct CollisionRect<STag, T>
where
    STag: SolidTag,
{
    pub id: Option<Index>,
    pub rect: Rect,
    /// Solid tag
    pub tag: Option<STag>,
    /// Optional, custom data.
    pub custom: Option<T>,
}

/// Builder struct for `CollisionRect`.
// #[derive(Default)]
pub struct CollisionRectBuilder<STag, T>
where
    STag: SolidTag,
{
    id:     Option<Index>,
    rect:   Rect,
    tag:    Option<STag>,
    custom: Option<T>,
}

impl<STag, T> CollisionRectBuilder<STag, T>
where
    STag: SolidTag,
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
    pub fn tag(mut self, tag: STag) -> Self {
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
            top:    pos.1,
            bottom: pos.1,
            left:   pos.0,
            right:  pos.0,
        };
        self
    }

    /// Infere the `rect: Rect` field by the given position and size;
    /// the position is the _center_ of the rect.
    pub fn with_pos_and_size(mut self, pos: Vector, size: Vector) -> Self {
        self.rect = Rect {
            top:    pos.1 + size.1 * 0.5,
            bottom: pos.1 - size.1 * 0.5,
            left:   pos.0 - size.0 * 0.5,
            right:  pos.0 + size.0 * 0.5,
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
    pub fn build(self) -> CollisionRect<STag, T> {
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

// NOTE: We have to implement `Default` manually like this because of the generics.
//       Rust doesn't seem to like `#[derive(Default)]` with generics.
impl<STag, T> Default for CollisionRectBuilder<STag, T>
where
    STag: SolidTag,
{
    fn default() -> Self {
        Self {
            id:     Default::default(),
            rect:   Default::default(),
            tag:    Default::default(),
            custom: Default::default(),
        }
    }
}

impl<STag, T> From<(Index, Vector, Option<Vector>)> for CollisionRect<STag, T>
where
    STag: SolidTag,
{
    fn from((id, pos, size_opt): (Index, Vector, Option<Vector>)) -> Self {
        CollisionRectBuilder::default()
            .id(id)
            .with_pos_and_maybe_size(pos, size_opt)
            .build()
    }
}

impl<STag, T> From<(Index, Vector, Option<Vector>, STag, Option<T>)>
    for CollisionRect<STag, T>
where
    STag: SolidTag,
{
    fn from(
        (id, pos, size_opt, tag, custom_opt): (
            Index,
            Vector,
            Option<Vector>,
            STag,
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
