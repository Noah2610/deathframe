use super::audio_manager::AudioManager;
use amethyst::audio::SourceHandle;
use core::amethyst;
use std::collections::HashMap;
use std::hash::Hash;

/// BGM song manager.
pub struct Songs<K>
where
    K: PartialEq + Eq + Hash,
{
    songs: HashMap<K, SourceHandle>,
}

impl<K> Songs<K>
where
    K: PartialEq + Eq + Hash,
{
    /// Returns the next song to play, for `amethyst_audio::DjSystem`.
    pub fn next_song(&mut self) -> Option<SourceHandle> {
        unimplemented!()
    }
}

impl<K> AudioManager<K> for Songs<K>
where
    K: PartialEq + Eq + Hash,
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
    K: PartialEq + Eq + Hash,
{
    fn default() -> Self {
        Self {
            songs: HashMap::new(),
        }
    }
}
