/// The playback state for a song.
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
    Finished,
}

impl Default for PlaybackState {
    fn default() -> Self {
        PlaybackState::Stopped
    }
}
