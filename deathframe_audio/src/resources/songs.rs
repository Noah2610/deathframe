use amethyst::assets::{AssetStorage, Loader};
use amethyst::audio::{
    FlacFormat,
    Mp3Format,
    OggFormat,
    Source,
    SourceHandle,
    WavFormat,
};
use core::amethyst;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::path::Path;

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
    /// Get the `SourceHandle` for the given key `K`.
    pub(crate) fn get_handle(&self, key: &K) -> Option<&SourceHandle> {
        self.songs.get(key)
    }
}
