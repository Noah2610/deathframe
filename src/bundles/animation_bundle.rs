use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::{DispatcherBuilder, World};
use animation::systems::prelude::*;
use core::amethyst;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

/// The `AnimationBundle` registers the following systems:
/// - `PlayAnimationsSystem` (named `"play_animations_system"`)
/// - `SwitchAnimationsSystem` (named `"switch_animations_system"`)
pub struct AnimationBundle<'a, AK>
where
    AK: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    deps: &'a [&'a str],
    _ak:  PhantomData<AK>,
}

impl<'a, AK> AnimationBundle<'a, AK>
where
    AK: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    /// Create new `AnimationBundle` with no dependencies.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set system dependencies for all registered systems.
    pub fn with_deps(mut self, deps: &'a [&'a str]) -> Self {
        self.deps = deps;
        self
    }
}

impl<'a, 'b, 'c, AK> SystemBundle<'a, 'b> for AnimationBundle<'c, AK>
where
    AK: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        builder.add(
            SwitchAnimationsSystem::<AK>::default(),
            "switch_animations_system",
            self.deps,
        );
        builder.add(
            PlayAnimationsSystem::default(),
            "play_animations_system",
            &[self.deps, &["switch_animations_system"]].concat(),
        );
        Ok(())
    }
}

impl<'a, AK> Default for AnimationBundle<'a, AK>
where
    AK: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    fn default() -> Self {
        Self {
            deps: Default::default(),
            _ak:  Default::default(),
        }
    }
}
