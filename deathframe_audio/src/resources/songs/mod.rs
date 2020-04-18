mod song;

pub use song::Song;

use super::helpers::load_audio;
use super::playback_state::PlaybackState;
use super::AudioSinkAction;
use amethyst::audio::AudioSink;
use amethyst::ecs::{World, WorldExt};
use core::amethyst;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;

/// BGM song manager.
pub struct Songs<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    pub(crate) songs: HashMap<K, Song>,
}

impl<K> Songs<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    /// Load song file for the given key, from the given path.
    /// The file format is derived from the filename's extension.
    /// Returns an Error, if no matching audio format was found
    /// for the file extension.
    /// Valid extensions: ".wav", ".mp3", ".ogg", ".flac"
    /// Panics if the default audio `Output` doesn't exist
    /// (`amethyst::audio::output::Output`).
    pub fn load_audio<P>(
        &mut self,
        key: K,
        path: P,
        world: &World,
    ) -> Result<(), String>
    where
        P: AsRef<Path>,
    {
        let source_handle =
            load_audio(path, &world.read_resource(), &world.read_resource())?;
        let audio_sink: AudioSink = AudioSink::new(&world.read_resource());
        self.songs.insert(key, Song::new(source_handle, audio_sink));
        Ok(())
    }

    /// Stops all `Song`s from playing.
    pub fn stop_all(&mut self) {
        for song in self.songs.values_mut() {
            song.stop();
        }
    }

    // TODO
    // /// Returns the next song to play, for `amethyst_audio::DjSystem`.
    // /// What is returned depends on the `PlaybackState` and `PlaybackBehavior`.
    // pub fn next_song(&mut self) -> Option<SourceHandle> {
    //     if let Some(playback_state) = self.playback_state.as_mut() {
    //         (match playback_state {
    //             PlaybackState::Stopped => None,
    //             PlaybackState::Playing(behavior) => {
    //                 next_song_for_behavior(behavior)
    //             }
    //             PlaybackState::Paused(_behavior) => None,
    //             PlaybackState::Finished => None,
    //         })
    //         .and_then(|key| self.get_source_handle(&key).cloned())
    //     } else {
    //         None
    //     }
    // }
}

// TODO
/// Returns the next song to play, depending on the `PlaybackBehavior`.
// fn next_song_for_behavior<K>(
//     playback_behavior: &mut PlaybackBehavior<K>,
// ) -> Option<K>
// where
//     K: PartialEq + Eq + Hash + Clone,
// {
//     match playback_behavior {
//         PlaybackBehavior::Autoplay(iter) => iter.next(),
//         PlaybackBehavior::Repeat(key) => Some(key.clone()),
//     }
// }

impl<K> Default for Songs<K>
where
    K: PartialEq + Eq + Hash + Clone,
{
    fn default() -> Self {
        Self {
            songs: HashMap::new(),
        }
    }
}
