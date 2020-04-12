use super::audio_manager::AudioManager;
use super::playback_behavior::PlaybackBehavior;
use super::playback_state::PlaybackState;
use amethyst::audio::SourceHandle;
use core::amethyst;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Cycle;
use std::vec::IntoIter;

/// BGM song manager.
/// Set the _playback order_ with one of the functions below,
/// otherwise no songs will play, even if you loaded them.
#[derive(Builder)]
#[builder(pattern = "owned", default)]
pub struct Songs<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    #[builder(setter(skip))]
    songs:             HashMap<K, SourceHandle>,
    volume:            f32,
    /// The order in which to play songs.
    playback_order:    Vec<K>,
    playback_state:    PlaybackState,
    playback_behavior: PlaybackBehavior,
    #[builder(setter(skip))]
    autoplay_queue:    Option<Cycle<IntoIter<K>>>,
}

impl<K> Songs<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    pub fn builder() -> SongsBuilder<K> {
        SongsBuilder {
            songs:             Default::default(),
            volume:            Default::default(),
            playback_order:    Default::default(),
            playback_state:    Default::default(),
            playback_behavior: Default::default(),
            autoplay_queue:    Default::default(),
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn set_playback_order(&mut self, order: Vec<K>) {
        self.playback_order = order;
    }

    pub fn set_playback_state(&mut self, state: PlaybackState) {
        self.playback_state = state;
    }

    pub fn set_playback_behavior(&mut self, state: PlaybackBehavior) {
        self.playback_behavior = state;
    }

    /// Returns the next song to play, for `amethyst_audio::DjSystem`.
    /// What is returned depends on the `PlaybackState` and `PlaybackBehavior`.
    pub fn next_song(&mut self) -> Option<SourceHandle> {
        match &self.playback_state {
            PlaybackState::Stopped => None,
            PlaybackState::Playing => self.next_song_for_behavior(),
            PlaybackState::Paused => None,
            PlaybackState::Finished => None,
        }
    }

    /// Returns the next song to play, depending on the `PlaybackBehavior`.
    /// Disregards the current `PlaybackState`.
    fn next_song_for_behavior(&mut self) -> Option<SourceHandle> {
        match &self.playback_behavior {
            PlaybackBehavior::Autoplay => {
                if self.autoplay_queue.is_none() {
                    self.autoplay_queue =
                        Some(self.playback_order.clone().into_iter().cycle());
                }
                if let Some(key) =
                    self.autoplay_queue.as_mut().and_then(Iterator::next)
                {
                    self.get_source_handle(&key).cloned()
                } else {
                    None
                }
            }
        }
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
            songs:             HashMap::new(),
            volume:            1.0,
            playback_order:    Vec::new(),
            playback_state:    PlaybackState::default(),
            playback_behavior: PlaybackBehavior::default(),
            autoplay_queue:    None,
        }
    }
}
