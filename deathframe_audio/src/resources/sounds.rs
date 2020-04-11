use super::audio_manager::AudioManager;
use amethyst::audio::SourceHandle;
use core::amethyst;
use std::collections::HashMap;
use std::hash::Hash;

/// Sound effects manager.
pub struct Sounds<K>
where
    K: PartialEq + Eq + Hash,
{
    sounds: HashMap<K, SourceHandle>,
}

impl<K> AudioManager<K> for Sounds<K>
where
    K: PartialEq + Eq + Hash,
{
    fn get_source_handle(&self, key: &K) -> Option<&SourceHandle> {
        self.sounds.get(key)
    }

    fn insert_source_handle(&mut self, key: K, source_handle: SourceHandle) {
        self.sounds.insert(key, source_handle);
    }
}

impl<K> Default for Sounds<K>
where
    K: PartialEq + Eq + Hash,
{
    fn default() -> Self {
        Self {
            sounds: HashMap::new(),
        }
    }
}
