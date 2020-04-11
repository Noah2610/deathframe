use super::audio_manager::AudioManager;
use amethyst::audio::SourceHandle;
use core::amethyst;
use std::collections::HashMap;
use std::hash::Hash;

/// BGM song manager.
pub struct Songs<'a, K>
where
    K: PartialEq + Eq + Hash,
{
    songs: HashMap<K, SourceHandle>,
    /// Pops-off and plays songs from the _end_ of the `Vec`,
    /// adds new songs to the queue by inserting them to the _start_ of the `Vec`.
    queue: Vec<&'a K>,
}

impl<'a, K> Songs<'a, K>
where
    K: PartialEq + Eq + Hash,
{
    /// Returns the next song to play, for `amethyst_audio::DjSystem`.
    pub fn next_song(&mut self) -> Option<SourceHandle> {
        self.queue
            .pop()
            .and_then(|key| self.get_source_handle(key).cloned())
    }
}

impl<'a, K> AudioManager<K> for Songs<'a, K>
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

impl<'a, K> Default for Songs<'a, K>
where
    K: PartialEq + Eq + Hash,
{
    fn default() -> Self {
        Self {
            songs: HashMap::new(),
            queue: Vec::new(),
        }
    }
}
