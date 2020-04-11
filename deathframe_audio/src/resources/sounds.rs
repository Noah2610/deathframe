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
    fn get_source_handles(&self) -> &HashMap<K, SourceHandle> {
        &self.sounds
    }

    fn mut_source_handles(&mut self) -> &mut HashMap<K, SourceHandle> {
        &mut self.sounds
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
