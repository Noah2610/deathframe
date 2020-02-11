use super::system_prelude::*;

/// If the velocity is smaller than or equal to this margin,
/// then just set the velocity to 0.0
const VELOCITY_MARGIN: f32 = 0.01;

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
                let vel = velocity.get(&axis);
                if vel > VELOCITY_MARGIN {
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
