use super::prelude::AnimationFrame;
use std::iter::Cycle;
use std::slice::Iter;

/// Type of animation frames iterator.
#[derive(Clone)]
pub enum AnimationFramesIter<'a> {
    /// Makes animation frames iterate endlessly.
    Cylce(Cycle<Iter<'a, AnimationFrame>>),
    /// Makes the animation play only once.
    Once(Iter<'a, AnimationFrame>),
}

impl<'a> From<Cycle<Iter<'a, AnimationFrame>>> for AnimationFramesIter<'a> {
    fn from(frames_iter: Cycle<Iter<'a, AnimationFrame>>) -> Self {
        AnimationFramesIter::Cycle(frames_iter)
    }
}

impl<'a> From<Iter<'a, AnimationFrame>> for AnimationFramesIter<'a> {
    fn from(frames_iter: Iter<'a, AnimationFrame>) -> Self {
        AnimationFramesIter::Once(frames_iter)
    }
}
