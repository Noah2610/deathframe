use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::{DispatcherBuilder, World};
use audio::systems::prelude::*;
use core::amethyst;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// The `AudioBundle` registers the following systems:
/// - `PlaySoundsSystem` (named `"play_sounds_system"`)
pub struct AudioBundle<'a, AK>
where
    AK: 'static + PartialEq + Eq + Hash + Send + Sync + Debug,
{
    deps: &'a [&'a str],
    _ak:  PhantomData<AK>,
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
}

impl<'a, 'b, 'c, AK> SystemBundle<'a, 'b> for AudioBundle<'c, AK>
where
    AK: 'static + PartialEq + Eq + Hash + Send + Sync + Debug,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        builder.add(
            PlaySoundsSystem::<AK>::default(),
            "play_sounds_system",
            self.deps,
        );
        Ok(())
    }
}

impl<'a, AK> Default for AudioBundle<'a, AK>
where
    AK: 'static + PartialEq + Eq + Hash + Send + Sync + Debug,
{
    fn default() -> Self {
        Self {
            deps: Default::default(),
            _ak:  Default::default(),
        }
    }
}
