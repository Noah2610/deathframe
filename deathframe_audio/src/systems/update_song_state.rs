use super::system_prelude::*;
use crate::resources::AudioSinkAction;
use std::hash::Hash;
use std::marker::PhantomData;

/// For now, this system only sets a `Song`'s `PlaybackState`
/// to `Stopped` if it's `AudioSink` is empty.
/// If the song is supposed to _loop_, then it is restarted
/// if it's `AudioSink` is empty.
pub struct UpdateSongStateSystem<K>
where
    K: PartialEq + Eq + Hash + Clone + Send + Sync,
{
    _k: PhantomData<K>,
}

impl<'a, K> System<'a> for UpdateSongStateSystem<K>
where
    K: 'static + PartialEq + Eq + Hash + Clone + Send + Sync,
{
    type SystemData = Write<'a, Songs<K>>;

    fn run(&mut self, mut songs: Self::SystemData) {
        for song in songs.songs.values_mut() {
            if let SongPlaybackState::Playing = &song.playback_state {
                if song.audio_sink.empty() {
                    if song.should_loop {
                        // The `UpdateSongPlaybackSystem` will restart the song.
                        song.audio_sink_action = Some(AudioSinkAction::Play);
                    } else {
                        song.playback_state = SongPlaybackState::Stopped;
                    }
                }
            }
        }
    }
}

impl<K> Default for UpdateSongStateSystem<K>
where
    K: PartialEq + Eq + Hash + Clone + Send + Sync,
{
    fn default() -> Self {
        Self {
            _k: Default::default(),
        }
    }
}
