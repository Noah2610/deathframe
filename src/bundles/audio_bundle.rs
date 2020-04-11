use amethyst::audio::AudioBundle as AmethystAudioBundle;
use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::{DispatcherBuilder, World};
use audio::systems::prelude::*;
use core::amethyst;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// The `AudioBundle` registers amehtyst's `amethyst_audio::AudioBundle`
/// and the following systems:
/// - `PlaySoundsSystem` (named `"play_sounds_system"`)
pub struct AudioBundle<'a, AK>
where
    AK: 'static + PartialEq + Eq + Hash + Send + Sync + Debug,
{
    sounds_default_volume: Option<f32>,
    deps:                  &'a [&'a str],
    _ak:                   PhantomData<AK>,
}

impl<'a, AK> AudioBundle<'a, AK>
where
    AK: 'static + PartialEq + Eq + Hash + Send + Sync + Debug,
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

impl<'a, 'b, 'c, AK> SystemBundle<'a, 'b> for AudioBundle<'c, AK>
where
    AK: 'static + PartialEq + Eq + Hash + Send + Sync + Debug,
{
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        AmethystAudioBundle::default().build(world, builder)?;

        let mut play_sounds_system = PlaySoundsSystem::<AK>::default();
        if let Some(sounds_default_volume) = self.sounds_default_volume {
            play_sounds_system =
                play_sounds_system.with_default_volume(sounds_default_volume);
        }
        builder.add(play_sounds_system, "play_sounds_system", self.deps);
        Ok(())
    }
}

impl<'a, AK> Default for AudioBundle<'a, AK>
where
    AK: 'static + PartialEq + Eq + Hash + Send + Sync + Debug,
{
    fn default() -> Self {
        Self {
            sounds_default_volume: None,
            deps:                  Default::default(),
            _ak:                   Default::default(),
        }
    }
}
