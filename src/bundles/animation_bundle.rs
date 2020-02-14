use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::{DispatcherBuilder, World};
use animation::systems::prelude::*;
use core::amethyst;

/// The `AnimationBundle` registers the following systems:
/// - `PlayAnimationsSystem` (named `"play_animations_system"`)
#[derive(Default)]
pub struct AnimationBundle<'a> {
    deps: &'a [&'a str],
}

impl<'a> AnimationBundle<'a> {
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

impl<'a, 'b, 'c> SystemBundle<'a, 'b> for AnimationBundle<'c> {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        builder.add(
            PlayAnimationsSystem::default(),
            "play_animations_system",
            self.deps,
        );
        Ok(())
    }
}
