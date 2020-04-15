use super::audio_manager::AudioManager;
use super::playback_behavior::PlaybackBehavior;
use super::playback_state::PlaybackState;
use amethyst::audio::SourceHandle;
use core::amethyst;
use std::collections::HashMap;
use std::hash::Hash;

/// BGM song manager.
pub struct Songs<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    songs:          HashMap<K, SourceHandle>,
    volume:         f32,
    playback_state: Option<PlaybackState<K>>,
}

impl<K> Songs<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    /// Plays the given song key on repeat.
    pub fn play(&mut self, key: K) {
        self.playback_state =
            Some(PlaybackState::Playing(PlaybackBehavior::Repeat(key)));
    }

    /// Plays the given ordered songs,
    /// looping back to the first song when the final song finishes.
    pub fn autoplay<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = K, IntoIter = std::vec::IntoIter<K>>,
    {
        self.playback_state = Some(PlaybackState::Playing(
            PlaybackBehavior::Autoplay(iter.into_iter().cycle()),
        ));
    }

    /// Pauses the playing song. Can only pause, if the `PlaybackState` is `Playing`.
    /// Returns an error if the state is not `Playing`.
    pub fn pause(&mut self) -> Result<(), String> {
        if let Some(PlaybackState::Playing(behavior)) =
            self.playback_state.take()
        {
            self.playback_state = Some(PlaybackState::Paused(behavior));
            Ok(())
        } else {
            Err(
                "Cannot pause `Songs` when it is not `PlaybackState::Playing`"
                    .into(),
            )
        }
    }

    /// Resumes playing from the `Paused` `PlaybackState`.
    /// Returns an error if the state is not `Paused`.
    pub fn resume(&mut self) -> Result<(), String> {
        if let Some(PlaybackState::Paused(behavior)) =
            self.playback_state.take()
        {
            self.playback_state = Some(PlaybackState::Playing(behavior));
            Ok(())
        } else {
            Err("Cannot play `Songs` when it is not `PlaybackState::Paused`"
                .into())
        }
    }

    /// Stops playing. Clears the `PlaybackBehavior`, so we'll need to
    /// start playing again with the `play` function.
    pub fn stop(&mut self) {
        self.playback_state = Some(PlaybackState::Stopped);
    }

    /// Returns the next song to play, for `amethyst_audio::DjSystem`.
    /// What is returned depends on the `PlaybackState` and `PlaybackBehavior`.
    pub fn next_song(&mut self) -> Option<SourceHandle> {
        if let Some(playback_state) = self.playback_state.as_mut() {
            (match playback_state {
                PlaybackState::Stopped => None,
                PlaybackState::Playing(behavior) => {
                    next_song_for_behavior(behavior)
                }
                PlaybackState::Paused(_behavior) => None,
                PlaybackState::Finished => None,
            })
            .and_then(|key| self.get_source_handle(&key).cloned())
        } else {
            None
        }
    }
}

/// Returns the next song to play, depending on the `PlaybackBehavior`.
fn next_song_for_behavior<K>(
    playback_behavior: &mut PlaybackBehavior<K>,
) -> Option<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    match playback_behavior {
        PlaybackBehavior::Autoplay(iter) => iter.next(),
        PlaybackBehavior::Repeat(key) => Some(key.clone()),
    }
}

impl<K> AudioManager<K> for Songs<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    fn get_source_handle(&self, key: &K) -> Option<&SourceHandle> {
        self.songs.get(key)
    }

    fn insert_source_handle(&mut self, key: K, source_handle: SourceHandle) {
        self.songs.insert(key, source_handle);
    }
}

impl<K> Default for Songs<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    fn default() -> Self {
        Self {
            songs:          HashMap::new(),
            volume:         1.0,
            playback_state: Some(PlaybackState::default()),
        }
    }
}
