pub trait CollisionCustomData: Send + Sync {}

impl<T> CollisionCustomData for T where T: Send + Sync
{
}
