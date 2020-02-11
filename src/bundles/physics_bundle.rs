use crate::systems::prelude::*;
use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::{DispatcherBuilder, World};
use core::amethyst;
use physics::collision::tag::CollisionTag;
use std::marker::PhantomData;

/// The `PhysicsBundle` will register the following systems:
/// - `MoveEntitiesSystem` (named `"move_entities_system"`)
/// - `UpdateCollisionsSystem` (named `"update_collisions_system"`)
/// - `ApplyBaseFrictionSystem` (named `"apply_base_friction_system"`)
/// - `ApplyGravitySystem` (named `"apply_gravity_system"`)
pub struct PhysicsBundle<'a, CU, CM>
where
    CU: 'static + CollisionTag,
    CM: 'static + CollisionTag,
{
    deps: &'a [&'a str],
    _cm:  PhantomData<CM>,
    _cu:  PhantomData<CU>,
}

impl<'a, CU, CM> PhysicsBundle<'a, CU, CM>
where
    CU: 'static + CollisionTag,
    CM: 'static + CollisionTag,
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

impl<'a, 'b, 'c, CU, CM> SystemBundle<'a, 'b> for PhysicsBundle<'c, CU, CM>
where
    CU: 'static + CollisionTag,
    CM: 'static + CollisionTag,
{
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        builder.add(
            ApplyGravitySystem::default(),
            "apply_gravity_system",
            self.deps,
        );
        builder.add(
            ApplyBaseFrictionSystem::default(),
            "apply_base_friction_system",
            &[self.deps, &["apply_gravity_system"]].concat(),
        );
        builder.add(
            MoveEntitiesSystem::<CM>::default(),
            "move_entities_system",
            &[self.deps, &[
                "apply_base_friction_system",
                "apply_gravity_system",
            ]]
            .concat(),
        );
        builder.add(
            UpdateCollisionsSystem::<CU>::default(),
            "update_collisions_system",
            &[self.deps, &["move_entities_system"]].concat(),
        );
        Ok(())
    }
}

impl<'a, CU, CM> Default for PhysicsBundle<'a, CU, CM>
where
    CU: 'static + CollisionTag,
    CM: 'static + CollisionTag,
{
    fn default() -> Self {
        Self {
            deps: Default::default(),
            _cm:  Default::default(),
            _cu:  Default::default(),
        }
    }
}
