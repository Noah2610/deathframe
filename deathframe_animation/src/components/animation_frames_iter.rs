use super::animation_frame::AnimationFrame;

/// An Iterator over `AnimationFrame`s.
pub trait AnimationFramesIter:
    Iterator<Item = AnimationFrame> + Send + Sync
{
}

impl<T> AnimationFramesIter for T where
    T: Iterator<Item = AnimationFrame> + Send + Sync
{
}
