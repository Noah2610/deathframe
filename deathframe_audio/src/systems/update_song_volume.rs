use super::system_prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

/// Handles updating the default audio
/// output sink's volume, used with `Songs`.
pub struct UpdateSongVolumeSystem<K>
where
    K: PartialEq + Eq + Hash + Clone + Send + Sync,
{
    prev_volumes: HashMap<K, f32>,
    _k:           PhantomData<K>,
}

impl<'a, K> System<'a> for UpdateSongVolumeSystem<K>
where
    K: 'static + PartialEq + Eq + Hash + Clone + Send + Sync,
{
    type SystemData = Write<'a, Songs<K>>;

    fn run(&mut self, mut songs: Self::SystemData) {
        for (key, song) in songs.songs.iter_mut() {
            let target_volume = song.get_volume();
            if self
                .prev_volumes
                .get(key)
                .map(|prev| prev != &target_volume)
                .unwrap_or(true)
            {
                self.prev_volumes.insert(key.clone(), target_volume);
                song.audio_sink.set_volume(target_volume);
            }
        }
    }
}

impl<K> Default for UpdateSongVolumeSystem<K>
where
    K: PartialEq + Eq + Hash + Clone + Send + Sync,
{
    fn default() -> Self {
        Self {
            prev_volumes: Default::default(),
            _k:           Default::default(),
        }
    }
}
