/// This tag is used for collision checking and when moving with solids.
/// Implement this trait for your own type to use with `Collider` and `Collidable`.
pub trait CollisionTag: Send + Sync + Clone + PartialEq {
    fn collides_with(&self, other: &Self) -> bool;
}

impl CollisionTag for () {
    fn collides_with(&self, _: &Self) -> bool {
        true
    }
}
