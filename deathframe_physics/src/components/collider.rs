use super::component_prelude::*;
use crate::collision::data::prelude::*;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Collider<T>
where
    T: 'static + CollisionTag,
{
    tag:  T,
    data: CollisionData,
}

impl<T> Collider<T>
where
    T: 'static + CollisionTag,
{
    pub fn new(tag: T) -> Self {
        Self {
            tag,
            data: Default::default(),
        }
    }
}
