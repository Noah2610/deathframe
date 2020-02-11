use super::system_prelude::*;

/// If the velocity is smaller than or equal to this margin,
/// then just set the velocity to 0.0
const VELOCITY_MARGIN: f32 = 0.001;

/// Constantly applies friction to entities with `BaseFriction`, for each axis.
/// Only if friction is enabled for the axis (see `BaseFriction`).
#[derive(Default)]
pub struct ApplyBaseFrictionSystem;

impl<'a> System<'a> for ApplyBaseFrictionSystem {
    type SystemData = (
        Read<'a, Time>,
        Entities<'a>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, BaseFriction>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            time,
            entities,
            mut velocities,
            mut base_frictions,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (_, velocity, base_friction) in
            (&entities, &mut velocities, &mut base_frictions)
                .join()
                .filter(|(entity, _, _)| {
                    is_entity_loaded(*entity, &loadables, &loadeds)
                })
        {
            Axis::for_each(|axis| {
                let vel = velocity.get(&axis);
                if vel.abs() > VELOCITY_MARGIN {
                    if let Some(fric) = base_friction.get(&axis) {
                        // Exponential function
                        // let reduced_vel = vel - vel * (fric * dt).exp();
                        let reduced_vel = vel - vel * fric * dt;
                        velocity.set(&axis, reduced_vel);
                    }
                } else {
                    // Velocity is too small, just set it to 0.0
                    velocity.clear(&axis);
                }
            });
        }
    }
}
