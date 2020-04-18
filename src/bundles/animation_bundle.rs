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
    name_suffix: Option<String>,
    deps:        &'a [&'a str],
    _ak:         PhantomData<AK>,
}

impl<'a, AK> AnimationBundle<'a, AK>
where
    AK: 'static + Hash + Eq + Send + Sync + Debug + Clone,
{
    /// Create new `AnimationBundle` with no dependencies.
    pub fn new() -> Self {
        Self::default()
    }

    /// Suffix all systems' names with the the given string.
    pub fn with_name_suffix<S>(mut self, name_suffix: S) -> Self
    where
        S: ToString,
    {
        self.name_suffix = Some(name_suffix.to_string());
        self
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
            &format!(
                "switch_animations_system{}",
                self.name_suffix
                    .as_ref()
                    .map(String::as_str)
                    .unwrap_or_default()
            ),
            self.deps,
        );
        builder.add(
            PlayAnimationsSystem::default(),
            &format!(
                "play_animations_system{}",
                self.name_suffix
                    .as_ref()
                    .map(String::as_str)
                    .unwrap_or_default()
            ),
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
            name_suffix: Default::default(),
            deps:        Default::default(),
            _ak:         Default::default(),
        }
    }
}
