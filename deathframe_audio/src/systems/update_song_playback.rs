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
    type SystemData = (
        Write<'a, Songs<K>>,
        Option<Write<'a, AudioSink>>,
        Option<Read<'a, Output>>,
    );

    fn run(
        &mut self,
        (mut songs, audio_sink_opt, output_opt): Self::SystemData,
    ) {
        if let Some(audio_sink_action) = songs.audio_sink_action.take() {
            if let (Some(mut audio_sink), Some(output)) =
                (audio_sink_opt, output_opt)
            {
                match audio_sink_action {
                    AudioSinkAction::Stop => {
                        audio_sink.stop();
                        *audio_sink = AudioSink::new(&output);
                    }
                    AudioSinkAction::Pause => audio_sink.pause(),
                    AudioSinkAction::Resume => audio_sink.play(),
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
