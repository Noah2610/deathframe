use super::component_prelude::*;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Collider<T>
where
    T: 'static + CollisionTag,
{
    tag: T,
}
