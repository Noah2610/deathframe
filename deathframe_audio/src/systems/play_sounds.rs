use super::system_prelude::*;
use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output as AudioOutput;
use amethyst::audio::Source;
use core::amethyst;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// Plays queued sounds from `SoundPlayer` components.
/// `SoundAction::Play` sounds are played with the _default volume_,
/// which can be set with the `with_default_volume` builder function.
/// See the `Default` implementation for the default.
pub struct PlaySoundsSystem<K>
where
    K: PartialEq + Eq + Hash,
{
    default_volume: f32,
    _k:             PhantomData<K>,
}

impl<K> PlaySoundsSystem<K>
where
    K: PartialEq + Eq + Hash,
{
    /// Sets the _default volume_ for sounds played with `SoundAction::Play`.
    pub fn with_default_volume(mut self, default_volume: f32) -> Self {
        self.default_volume = default_volume;
        self
    }
}

impl<'a, K> System<'a> for PlaySoundsSystem<K>
where
    K: 'static + PartialEq + Eq + Hash + Send + Sync + Debug,
{
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, SoundPlayer<K>>,
        ReadExpect<'a, Sounds<K>>,
        Read<'a, AssetStorage<Source>>,
        Read<'a, AudioOutput>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut sound_player_store,
            sounds,
            asset_storage,
            audio_output,
        ): Self::SystemData,
    ) {
        for (_entity, sound_player) in
            (&entities, &mut sound_player_store).join()
        {
            for action in sound_player.drain_actions() {
                match action {
                    SoundAction::Play(sound_key) => {
                        play_sound(
                            &sounds,
                            &asset_storage,
                            &audio_output,
                            &sound_key,
                            self.default_volume,
                        );
                    }
                    SoundAction::PlayWithVolume(sound_key, volume) => {
                        play_sound(
                            &sounds,
                            &asset_storage,
                            &audio_output,
                            &sound_key,
                            volume,
                        );
                    }
                }
            }
        }
    }
}

fn play_sound<K>(
    sounds: &Sounds<K>,
    asset_storage: &AssetStorage<Source>,
    audio_output: &AudioOutput,
    sound_key: &K,
    volume: f32,
) where
    K: PartialEq + Eq + Hash + Debug,
{
    if let Some(sound_source) = sounds.get_handle(sound_key) {
        if let Some(sound) = asset_storage.get(sound_source) {
            audio_output.play_once(sound, volume);
        } else {
            eprintln!(
                "[WARNING]\n    Sound source for key {:?} is not loaded in \
                 asset storage",
                sound_key
            );
        }
    } else {
        eprintln!(
            "[WARNING]\n    Sound source for key {:?} is not registered",
            sound_key
        );
    }
}

impl<K> Default for PlaySoundsSystem<K>
where
    K: PartialEq + Eq + Hash,
{
    fn default() -> Self {
        Self {
            default_volume: 1.0,
            _k:             Default::default(),
        }
    }
}
