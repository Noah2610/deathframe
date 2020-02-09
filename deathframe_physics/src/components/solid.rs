use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
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
