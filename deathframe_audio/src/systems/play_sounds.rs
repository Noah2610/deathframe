use super::system_prelude::*;
use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output as AudioOutput;
use amethyst::audio::Source;
use core::amethyst;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

// TODO
const DEFAULT_VOLUME: f32 = 1.0;

#[derive(Default)]
pub struct PlaySoundsSystem<K>
where
    K: PartialEq + Eq + Hash,
{
    _k: PhantomData<K>,
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
        for (entity, sound_player) in
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
                            DEFAULT_VOLUME,
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
