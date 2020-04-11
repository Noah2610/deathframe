/// The playback behavior for the `Songs` BGM manager.
#[derive(Clone)]
pub enum PlaybackBehavior {
    /// Play the songs in the `Songs`' _playback order_.
    Autoplay,
}

impl Default for PlaybackBehavior {
    fn default() -> Self {
        PlaybackBehavior::Autoplay
    }
}
