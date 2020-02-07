/// This tag is used for collision checking and when moving with solids.
/// Implement this trait for your own type to use with `Collider` and `Collidable`.
/// This trait is automatically implemented for all types
/// implementing `PartialEq`. For those, the `collides_with` function
/// simply checks for equality between both types.
pub trait CollisionTag: Send + Sync {
    fn collides_with(&self, other: &Self) -> bool;
}

impl<T> CollisionTag for T
where
    T: Send + Sync + PartialEq,
{
    fn collides_with(&self, other: &Self) -> bool {
        self == other
    }
}
