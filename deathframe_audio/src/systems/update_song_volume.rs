use super::system_prelude::*;
use core::amethyst::audio::AudioSink;
use std::hash::Hash;
use std::marker::PhantomData;

/// Handles updating the default audio
/// output sink's volume, used with `Songs`.
pub struct UpdateSongVolumeSystem<K>
where
    K: PartialEq + Eq + Hash + Clone + Send + Sync,
{
    prev_volume: Option<f32>,
    _k:          PhantomData<K>,
}

impl<'a, K> System<'a> for UpdateSongVolumeSystem<K>
where
    K: 'static + PartialEq + Eq + Hash + Clone + Send + Sync,
{
    type SystemData =
        (Option<Read<'a, Songs<K>>>, Option<Write<'a, AudioSink>>);

    fn run(&mut self, (songs_opt, audio_sink_opt): Self::SystemData) {
        if let Some(songs) = songs_opt {
            let target_volume = songs.get_volume();
            if self
                .prev_volume
                .map(|prev| prev != target_volume)
                .unwrap_or(true)
            {
                self.prev_volume = Some(target_volume);
                if let Some(mut audio_sink) = audio_sink_opt {
                    audio_sink.set_volume(target_volume);
                } else {
                    eprintln!("[WARNING]\n    No audio sink");
                }
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
            prev_volume: Default::default(),
            _k:          Default::default(),
        }
    }
}
