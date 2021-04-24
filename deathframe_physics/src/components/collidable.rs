use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields, from = "C")]
pub struct Collidable<C>
where
    C: 'static + CollisionTag,
{
    pub tag: C,
}

impl<C> Collidable<C>
where
    C: 'static + CollisionTag,
{
    pub fn new(tag: C) -> Self {
        Self { tag }
    }
}

impl<C> WithCollisionTag<C> for Collidable<C>
where
    C: CollisionTag,
{
    fn collision_tag(&self) -> &C {
        &self.tag
    }
}

impl<C> From<C> for Collidable<C>
where
    C: CollisionTag,
{
    fn from(tag: C) -> Self {
        Self { tag }
    }
}
