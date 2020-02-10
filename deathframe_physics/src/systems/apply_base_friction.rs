use super::system_prelude::*;

/// Constantly applies friction to entities with `BaseFriction`, for each axis.
/// Only if friction is enabled for the axis (see `BaseFriction`).
#[derive(Default)]
pub struct ApplyBaseFrictionSystem;

impl<'a> System<'a> for ApplyBaseFrictionSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, BaseFriction>,
    );

    fn run(
        &mut self,
        (time, mut velocities, mut base_frictions): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (velocity, base_friction) in
            (&mut velocities, &mut base_frictions).join()
        {
            Axis::for_each(|axis| {
                if let Some(fric) = base_friction.get(&axis) {
                    // Exponential function
                    let reduced_vel = velocity.get(&axis) * (fric * dt).exp();
                    velocity.set(&axis, reduced_vel);
                }
            });
        }
    }
}
