use super::AudioSinkAction;
use super::PlaybackState;
use amethyst::audio::{AudioSink, SourceHandle};
use core::amethyst;

const DEFAULT_VOLUME: f32 = 1.0;

/// A `Song`, wraps it's `SourceHandle` and `AudioSink`.
pub struct Song {
    pub(crate) source:            SourceHandle,
    pub(crate) playback_state:    PlaybackState,
    volume:                       f32,
    pub(crate) should_loop:       bool,
    pub(crate) audio_sink:        AudioSink,
    pub(crate) audio_sink_action: Option<AudioSinkAction>,
}

impl Song {
    /// Creates a new `Song` with the given `SourceHandle` and `AudioSink`.
    pub fn new(source: SourceHandle, audio_sink: AudioSink) -> Self {
        Self {
            source,
            playback_state: Default::default(),
            volume: DEFAULT_VOLUME,
            should_loop: false,
            audio_sink,
            audio_sink_action: Default::default(),
        }
    }

    pub fn with_loop(mut self, should_loop: bool) -> Self {
        self.should_loop = should_loop;
        self
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    /// Plays the song.
    pub fn play(&mut self) {
        self.audio_sink_action = Some(AudioSinkAction::Play);
        self.playback_state = PlaybackState::Playing;
    }

    /// Pauses the Song. Can only pause, if the `PlaybackState` is `Playing`.
    /// Returns an error if the state is not `Playing`.
    pub fn pause(&mut self) -> Result<(), String> {
        if let PlaybackState::Playing = &self.playback_state {
            self.audio_sink_action = Some(AudioSinkAction::Pause);
            self.playback_state = PlaybackState::Paused;
            Ok(())
        } else {
            Err(
                "Cannot pause `Songs` when it is not `PlaybackState::Playing`"
                    .into(),
            )
        }
    }

    /// Resumes playing from the `Paused` `PlaybackState`.
    /// Returns an error if the state is not `Paused`.
    pub fn resume(&mut self) -> Result<(), String> {
        if let PlaybackState::Paused = &self.playback_state {
            self.audio_sink_action = Some(AudioSinkAction::Resume);
            self.playback_state = PlaybackState::Playing;
            Ok(())
        } else {
            Err(
                "Cannot resume `Songs` when it is not `PlaybackState::Paused`"
                    .into(),
            )
        }
    }

    /// Stops playing the song.
    pub fn stop(&mut self) {
        self.audio_sink_action = Some(AudioSinkAction::Stop);
        self.playback_state = PlaybackState::Stopped;
    }

    /// Returns `true` if the song is playing.
    pub fn is_playing(&self) -> bool {
        if let PlaybackState::Playing = &self.playback_state {
            true
        } else {
            false
        }
    }
}
