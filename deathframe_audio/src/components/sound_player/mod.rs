pub mod prelude {
    pub use super::sound_action::SoundAction;
    pub use super::sound_player::SoundPlayer;
}

mod sound_action;
mod sound_player;

use super::component_prelude;
