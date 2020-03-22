use super::prelude::AnimationFrame;
use std::iter::Cycle;
use std::vec::IntoIter;

/// Type of animation frames iterator.
#[derive(Clone)]
pub enum AnimationFramesIter {
    /// Makes animation frames iterate endlessly.
    Cycle(Cycle<IntoIter<AnimationFrame>>),
    /// Makes the animation play only once.
    Once(IntoIter<AnimationFrame>),
}

impl Iterator for AnimationFramesIter {
    type Item = AnimationFrame;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            AnimationFramesIter::Cycle(iter) => iter.next(),
            AnimationFramesIter::Once(iter) => iter.next(),
        }
    }
}

impl From<Cycle<IntoIter<AnimationFrame>>> for AnimationFramesIter {
    fn from(frames_iter: Cycle<IntoIter<AnimationFrame>>) -> Self {
        AnimationFramesIter::Cycle(frames_iter)
    }
}

impl From<IntoIter<AnimationFrame>> for AnimationFramesIter {
    fn from(frames_iter: IntoIter<AnimationFrame>) -> Self {
        AnimationFramesIter::Once(frames_iter)
    }
}
