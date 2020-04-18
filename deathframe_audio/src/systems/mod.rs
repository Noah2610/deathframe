pub mod prelude {
    pub use super::play_sounds::PlaySoundsSystem;
    pub use super::update_song_playback::UpdateSongPlaybackSystem;
    pub use super::update_song_state::UpdateSongStateSystem;
    pub use super::update_song_volume::UpdateSongVolumeSystem;
}

mod system_prelude {
    pub(super) use crate::components::prelude::*;
    pub(super) use crate::resources::prelude::*;
    pub(super) use core::amethyst::audio::AudioSink;
    pub(super) use core::systems::system_prelude::*;
}

mod play_sounds;
mod update_song_playback;
mod update_song_state;
mod update_song_volume;
