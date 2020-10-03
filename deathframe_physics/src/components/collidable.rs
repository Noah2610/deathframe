use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct Collidable<T>
where
    T: 'static + CollisionTag,
{
    pub tag: T,
}

impl<T> Collidable<T>
where
    T: 'static + CollisionTag,
{
    pub fn new(tag: T) -> Self {
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
