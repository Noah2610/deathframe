use amethyst::audio::{AudioBundle as AmethystAudioBundle, DjSystemDesc};
use amethyst::core::bundle::SystemBundle;
use amethyst::core::SystemDesc;
use amethyst::ecs::{DispatcherBuilder, World};
use audio::resources::prelude::Songs;
use audio::systems::prelude::*;
use core::amethyst;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// The `AudioBundle` registers amethyst's `amethyst_audio::AudioBundle`
/// and the following systems:
/// - `amethyst_audio::DjSystem` (named `"dj_system"`)
///   which will use `resources::prelude::Songs` BGM manager,
///   if it has been inserted into the world.
/// - `PlaySoundsSystem` (named `"play_sounds_system"`)
/// - `UpdateSongVolumeSystem` (named `"update_song_volume_system"`)
/// - `UpdateSongPlaybackSystem` (named `"update_song_playback_system"`)
///
/// Both generics are used for both the `Sounds` and the `Songs` audio keys.
/// `KA` for `Sounds`, `KB` for `Songs`.
pub struct AudioBundle<'a, KA, KB>
where
    KA: 'static + PartialEq + Eq + Hash + Clone + Send + Sync + Debug,
    KB: 'static + PartialEq + Eq + Hash + Clone + Send + Sync + Debug,
{
    sounds_default_volume: Option<f32>,
    deps:                  &'a [&'a str],
    _ka:                   PhantomData<KA>,
    _kb:                   PhantomData<KB>,
}

impl<'a, KA, KB> AudioBundle<'a, KA, KB>
where
    KA: 'static + PartialEq + Eq + Hash + Clone + Send + Sync + Debug,
    KB: 'static + PartialEq + Eq + Hash + Clone + Send + Sync + Debug,
{
    /// Create new `AudioBundle` with no dependencies.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set system dependencies for all registered systems.
    pub fn with_deps(mut self, deps: &'a [&'a str]) -> Self {
        self.deps = deps;
        self
    }

    /// Set the _default volume_ for the `PlaySoundsSystem`.
    /// Check the system's documentation for more info.
    pub fn with_sounds_default_volume(mut self, default_volume: f32) -> Self {
        self.sounds_default_volume = Some(default_volume);
        self
    }
}

impl<'a, 'b, 'c, KA, KB> SystemBundle<'a, 'b> for AudioBundle<'c, KA, KB>
where
    KA: 'static + PartialEq + Eq + Hash + Clone + Send + Sync + Debug,
    KB: 'static + PartialEq + Eq + Hash + Clone + Send + Sync + Debug,
{
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        AmethystAudioBundle::default().build(world, builder)?;

        builder.add(
            DjSystemDesc::new(|songs: &mut Songs<KB>| songs.next_song())
                .build(world),
            "dj_system",
            self.deps,
        );

        let mut play_sounds_system = PlaySoundsSystem::<KA>::default();
        if let Some(sounds_default_volume) = self.sounds_default_volume {
            play_sounds_system =
                play_sounds_system.with_default_volume(sounds_default_volume);
        }
        builder.add(play_sounds_system, "play_sounds_system", self.deps);

        builder.add(
            UpdateSongVolumeSystem::<KB>::default(),
            "update_song_volume_system",
            self.deps,
        );

        builder.add(
            UpdateSongPlaybackSystem::<KB>::default(),
            "update_song_playback_system",
            self.deps,
        );

        Ok(())
    }
}

impl<'a, KA, KB> Default for AudioBundle<'a, KA, KB>
where
    KA: 'static + PartialEq + Eq + Hash + Clone + Send + Sync + Debug,
    KB: 'static + PartialEq + Eq + Hash + Clone + Send + Sync + Debug,
{
    fn default() -> Self {
        Self {
            sounds_default_volume: None,
            deps:                  Default::default(),
            _ka:                   Default::default(),
            _kb:                   Default::default(),
        }
    }
}
