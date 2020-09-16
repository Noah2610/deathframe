use super::component_prelude::*;

#[derive(Component, Deserialize)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct Solid<C>
where
    C: 'static + CollisionTag,
{
    pub tag: C,
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
