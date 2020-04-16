use super::system_prelude::*;
use std::hash::Hash;
use std::marker::PhantomData;
use SongPlaybackState as PlaybackState;

pub struct UpdateSongPlaybackSystem<K>
where
    K: PartialEq + Eq + Hash + Clone + Send + Sync,
{
    prev_state: Option<PlaybackStateWrapper>,
    _k:         PhantomData<K>,
}

impl<'a, K> System<'a> for UpdateSongPlaybackSystem<K>
where
    K: 'static + PartialEq + Eq + Hash + Clone + Send + Sync,
{
    type SystemData =
        (Option<Read<'a, Songs<K>>>, Option<Write<'a, AudioSink>>);

    fn run(&mut self, (songs_opt, audio_sink_opt): Self::SystemData) {
        use AudioSinkAction as Action;
        use PlaybackStateWrapper as Wrapper;

        if let Some(songs) = songs_opt.as_ref() {
            if let Some(curr_state) = songs
                .playback_state
                .as_ref()
                .map(|state| -> PlaybackStateWrapper { state.into() })
            {
                let prev_state = self.prev_state.get_or_insert(curr_state);

                if *prev_state != curr_state {
                    let audio_sink_action_opt = match &curr_state {
                        Wrapper::Stopped => Some(Action::Stop),
                        Wrapper::Playing => match prev_state {
                            Wrapper::Stopped => None,
                            Wrapper::Playing => unreachable!(),
                            Wrapper::Paused => Some(Action::Resume),
                            Wrapper::Finished => None,
                        },
                        Wrapper::Paused => Some(Action::Pause),
                        Wrapper::Finished => None,
                    };

                    if let Some(audio_sink_action) = audio_sink_action_opt {
                        if let Some(audio_sink) = audio_sink_opt {
                            match audio_sink_action {
                                AudioSinkAction::Stop => audio_sink.stop(),
                                AudioSinkAction::Pause => audio_sink.pause(),
                                AudioSinkAction::Resume => audio_sink.play(),
                            }
                        }
                    }

                    self.prev_state = Some(curr_state);
                }
            }
        }
    }
}

enum AudioSinkAction {
    Stop,
    Pause,
    Resume,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum PlaybackStateWrapper {
    Stopped,
    Playing,
    Paused,
    Finished,
}

impl<K> Into<PlaybackStateWrapper> for &PlaybackState<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    fn into(self) -> PlaybackStateWrapper {
        use PlaybackStateWrapper as Wrapper;

        match self {
            PlaybackState::Stopped => Wrapper::Stopped,
            PlaybackState::Playing(_) => Wrapper::Playing,
            PlaybackState::Paused(_) => Wrapper::Paused,
            PlaybackState::Finished => Wrapper::Finished,
        }
    }
}

impl<K> Default for UpdateSongPlaybackSystem<K>
where
    K: PartialEq + Eq + Hash + Clone + Send + Sync,
{
    fn default() -> Self {
        Self {
            prev_state: Default::default(),
            _k:         Default::default(),
        }
    }
}
