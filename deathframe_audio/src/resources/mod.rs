pub mod prelude {
    pub use super::playback_state::PlaybackState as SongPlaybackState;
    pub use super::songs::{Song, Songs};
    pub use super::sounds::Sounds;
}

mod playback_state;
mod songs;
mod sounds;

mod helpers;

pub(crate) enum AudioSinkAction {
    Play,
    Stop,
    Pause,
    Resume,
}
