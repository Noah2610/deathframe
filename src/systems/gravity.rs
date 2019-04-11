use super::system_prelude::*;

/// This system increases entities' velocities every frame.
pub struct GravitySystem;

impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Gravity>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
    );

    fn run(
        &mut self,
        (time, gravities, mut velocities, mut decr_velocities): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (gravity, velocity, mut decr_velocity) in
            (&gravities, &mut velocities, (&mut decr_velocities).maybe()).join()
        {
            if gravity.x != 0.0 {
                velocity.x += gravity.x * dt;
                decr_velocity.as_mut().map(|decr| {
                    if gravity.x > 0.0 {
                        decr.should_decrease_x_pos = false;
                    } else if gravity.x < 0.0 {
                        decr.should_decrease_x_neg = false;
                    }
                });
            }
            if gravity.y != 0.0 {
                velocity.y += gravity.y * dt;
                decr_velocity.as_mut().map(|decr| {
                    if gravity.y > 0.0 {
                        decr.should_decrease_y_pos = false;
                    } else if gravity.y < 0.0 {
                        decr.should_decrease_y_neg = false;
                    }
                });
            }
        }
    }
}
