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
    /// Returns a reference to the `Song` for the given key.
    pub fn get(&self, key: &K) -> Option<&Song> {
        self.songs.get(key)
    }

    /// Returns a mutable reference to the `Song` for the given key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut Song> {
        self.songs.get_mut(key)
    }

    /// Plays the song for the given key, if it exists.
    pub fn play(&mut self, key: &K) {
        self.get_mut(key).map(Song::play);
    }

    /// Pauses the song for the given key, if it exists and is playing.
    /// Returns an error if the state is not `Playing`.
    pub fn pause(&mut self, key: &K) -> Result<(), String> {
        self.get_mut(key).map(Song::pause).unwrap_or(Ok(()))
    }

    /// Resumes the song for the given key, if it exists and is paused.
    /// Returns an error if the state is not `Paused`.
    pub fn resume(&mut self, key: &K) -> Result<(), String> {
        self.get_mut(key).map(Song::resume).unwrap_or(Ok(()))
    }

    /// Stops the song for the given key, if it exists.
    pub fn stop(&mut self, key: &K) {
        self.get_mut(key).map(Song::stop);
    }

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
        does_loop: bool,
        world: &World,
    ) -> Result<(), String>
    where
        P: AsRef<Path>,
    {
        let source_handle =
            load_audio(path, &world.read_resource(), &world.read_resource())?;
        let audio_sink: AudioSink = AudioSink::new(&world.read_resource());
        let song = Song::new(source_handle, audio_sink).with_loop(does_loop);
        self.songs.insert(key, song);
        Ok(())
    }

    /// Stops all `Song`s from playing.
    pub fn stop_all(&mut self) {
        for song in self.songs.values_mut() {
            song.stop();
        }
    }
}

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
