use super::helpers::load_audio;
use amethyst::audio::SourceHandle;
use amethyst::ecs::{World, WorldExt};
use core::amethyst;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

/// Sound effects manager.
pub struct Sounds<K>
where
    K: PartialEq + Eq + Hash,
{
    sounds: HashMap<K, SourceHandle>,
}

impl<K> Sounds<K>
where
    K: PartialEq + Eq + Hash,
{
    /// Load sound file for the given key, from the given path.
    /// The file format is derived from the filename's extension.
    /// Returns an Error, if no matching audio format was found
    /// for the file extension.
    /// Valid extensions: ".wav", ".mp3", ".ogg", ".flac"
    pub fn load_audio<P>(
        &mut self,
        key: K,
        path: P,
        world: &World,
    ) -> Result<(), String>
    where
        P: AsRef<Path>,
    {
        self.sounds.insert(
            key,
            load_audio(path, &world.read_resource(), &world.read_resource())?,
        );
        Ok(())
    }

    pub(crate) fn get_source_handle(&self, key: &K) -> Option<&SourceHandle> {
        self.sounds.get(key)
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
