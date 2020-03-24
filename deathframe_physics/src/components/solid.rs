use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Solid<C>
where
    C: 'static + CollisionTag,
{
    pub(crate) tag: C,
}

impl<C> Solid<C>
where
    C: CollisionTag,
{
    pub fn new(tag: C) -> Self {
        Self { tag }
    }
}

impl<C> WithCollisionTag<C> for Solid<C>
where
    C: CollisionTag,
{
    fn collision_tag(&self) -> &C {
        &self.tag
    }
}
