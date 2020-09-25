use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::{DispatcherBuilder, World};
use core::amethyst;
use physics::collision::tag::CollisionTag;
use physics::systems::prelude::*;
use std::marker::PhantomData;

/// The `PhysicsBundle` registers the following systems:
/// - `MoveEntitiesSystem` (named `"move_entities_system"`)
/// - `UpdateCollisionsSystem` (named `"update_collisions_system"`)
/// - `ApplyBaseFrictionSystem` (named `"apply_base_friction_system"`)
/// - `ApplyGravitySystem` (named `"apply_gravity_system"`)
/// - `HandleTakingDamageSystem` (named `"handle_taking_damage_system"`)
pub struct PhysicsBundle<'a, CU, CM>
where
    CU: 'static + CollisionTag,
    CM: 'static + CollisionTag,
{
    deps:                                &'a [&'a str],
    apply_base_friction_velocity_margin: Option<f32>,
    _cm:                                 PhantomData<CM>,
    _cu:                                 PhantomData<CU>,
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

    /// Set system dependencies for all registered systems.
    pub fn with_deps(mut self, deps: &'a [&'a str]) -> Self {
        self.deps = deps;
        self
    }

    /// Set the `ApplyBaseFrictionSystem`'s `velocity_margin`.
    /// See the `ApplyBaseFrictionSystem::with_velocity_margin` function.
    pub fn with_apply_base_friciton_velocity_margin(
        mut self,
        velocity_margin: f32,
    ) -> Self {
        self.apply_base_friction_velocity_margin = Some(velocity_margin);
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
            {
                let system = ApplyBaseFrictionSystem::default();
                if let Some(velocity_margin) =
                    self.apply_base_friction_velocity_margin
                {
                    system.with_velocity_margin(velocity_margin)
                } else {
                    system
                }
            },
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
        builder.add(
            HandleTakingDamageSystem::<CU>::default(),
            "handle_taking_damage_system",
            &[self.deps, &["update_collisions_system"]].concat(),
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
            deps:                                Default::default(),
            apply_base_friction_velocity_margin: Default::default(),
            _cm:                                 Default::default(),
            _cu:                                 Default::default(),
        }
    }
}
