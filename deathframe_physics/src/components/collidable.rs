use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Collidable<T>
where
    T: 'static + CollisionTag,
{
    pub(crate) tag: T,
}

impl<T> Collidable<T>
where
    T: 'static + CollisionTag,
{
    pub fn new(tag: T) -> Self {
        Self { tag }
    }
}
