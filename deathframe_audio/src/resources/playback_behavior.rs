use std::hash::Hash;
use std::iter::Cycle;
use std::vec::IntoIter;

/// The playback behavior for the `Songs` BGM manager.
#[derive(Clone)]
pub enum PlaybackBehavior<K>
where
    K: PartialEq + Eq + Hash,
{
    /// Play the songs in the given order.
    Autoplay(Cycle<IntoIter<K>>),

    /// Play the given song, on repeat.
    Repeat(K),
}
