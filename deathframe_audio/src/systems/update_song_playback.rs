use super::system_prelude::*;
use crate::resources::AudioSinkAction;
use core::amethyst::audio::output::Output;
use std::hash::Hash;
use std::marker::PhantomData;

pub struct UpdateSongPlaybackSystem<K>
where
    K: PartialEq + Eq + Hash + Clone + Send + Sync,
{
    _k: PhantomData<K>,
}

impl<'a, K> System<'a> for UpdateSongPlaybackSystem<K>
where
    K: 'static + PartialEq + Eq + Hash + Clone + Send + Sync,
{
    type SystemData = (Write<'a, Songs<K>>, Read<'a, Output>);

    fn run(&mut self, (mut songs, output): Self::SystemData) {
        for song in songs.songs.values_mut() {
            if let Some(audio_sink_action) = song.audio_sink_action.take() {
                match audio_sink_action {
                    AudioSinkAction::Stop => {
                        song.audio_sink.stop();
                        song.audio_sink = AudioSink::new(&output);
                    }
                    AudioSinkAction::Pause => song.audio_sink.pause(),
                    AudioSinkAction::Resume => song.audio_sink.play(),
                }
            }
        }
    }
}

impl<K> Default for UpdateSongPlaybackSystem<K>
where
    K: PartialEq + Eq + Hash + Clone + Send + Sync,
{
    fn default() -> Self {
        Self {
            _k: Default::default(),
        }
    }
}
