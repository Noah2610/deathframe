use super::system_prelude::*;

/// Gets all entities with `Velocity` and `DecreaseVelocity`,
/// and decreases their velocities every frame.
pub struct DecreaseVelocitiesSystem;

impl<'a> System<'a> for DecreaseVelocitiesSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (time, mut decr_velocities, mut velocities): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (decr, velocity) in (&mut decr_velocities, &mut velocities).join() {
            let signx = velocity.x.signum();
            let signy = velocity.y.signum();

            // X
            if velocity.x != 0.0 {
                if (signx > 0.0 && decr.should_decrease_x_pos)
                    || (signx < 0.0 && decr.should_decrease_x_neg)
                {
                    velocity.x -= (decr.x * dt) * signx;
                }
            }
            if velocity.x.signum() != signx {
                velocity.x = 0.0;
            }
            decr.should_decrease_x_pos = true;
            decr.should_decrease_x_neg = true;

            // Y
            if velocity.y != 0.0 {
                if (signy > 0.0 && decr.should_decrease_y_pos)
                    || (signy < 0.0 && decr.should_decrease_y_neg)
                {
                    velocity.y -= (decr.y * dt) * signy;
                }
            }
            if velocity.y.signum() != signy {
                velocity.y = 0.0;
            }
            decr.should_decrease_y_pos = true;
            decr.should_decrease_y_neg = true;
        }
    }
}
