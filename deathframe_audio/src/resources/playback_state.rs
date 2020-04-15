use super::playback_behavior::PlaybackBehavior;
use std::hash::Hash;

/// The playback state for a song.
pub enum PlaybackState<K>
where
    K: PartialEq + Eq + Hash,
{
    Stopped,
    Playing(PlaybackBehavior<K>),
    Paused(PlaybackBehavior<K>),
    Finished,
}

impl<K> Default for PlaybackState<K>
where
    K: PartialEq + Eq + Hash,
{
    fn default() -> Self {
        PlaybackState::Stopped
    }
}
