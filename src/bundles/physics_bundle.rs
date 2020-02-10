use crate::systems::prelude::*;
use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::{DispatcherBuilder, World};
use core::amethyst;
use physics::collision::tag::CollisionTag;
use std::marker::PhantomData;

/// The `PhysicsBundle` will register
/// the `MoveEntitiesSystem` (named `"move_entities_system"`) and
/// the `UpdateCollisionsSystem` (named `"update_collisions_system"`).
pub struct PhysicsBundle<'a, CM, CU>
where
    CM: 'static + CollisionTag,
    CU: 'static + CollisionTag,
{
    deps: &'a [&'a str],
    _cm:  PhantomData<CM>,
    _cu:  PhantomData<CU>,
}

impl<'a, CM, CU> PhysicsBundle<'a, CM, CU>
where
    CM: 'static + CollisionTag,
    CU: 'static + CollisionTag,
{
    /// Create new `PhysicsBundle` with no dependencies.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set system dependencies for both registered physics systems.
    pub fn with_deps(mut self, deps: &'a [&'a str]) -> Self {
        self.deps = deps;
        self
    }
}

impl<'a, 'b, 'c, CM, CU> SystemBundle<'a, 'b> for PhysicsBundle<'c, CM, CU>
where
    CM: 'static + CollisionTag,
    CU: 'static + CollisionTag,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        builder.add(
            MoveEntitiesSystem::<CM>::default(),
            "move_entities_system",
            self.deps,
        );
        builder.add(
            UpdateCollisionsSystem::<CU>::default(),
            "update_collisions_system",
            self.deps,
        );
        Ok(())
    }
}

impl<'a, CM, CU> Default for PhysicsBundle<'a, CM, CU>
where
    CM: 'static + CollisionTag,
    CU: 'static + CollisionTag,
{
    fn default() -> Self {
        Self {
            deps: Default::default(),
            _cm:  Default::default(),
            _cu:  Default::default(),
        }
    }
}
