use super::system_prelude::*;

/// This system increases entities' velocities every frame.
#[derive(Default)]
pub struct GravitySystem;

impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Gravity>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
    );

    fn run(
        &mut self,
        (
            time,
            gravities,
            loadables,
            loadeds,
            mut velocities,
            mut decr_velocities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (gravity, velocity, mut decr_velocity, loadable_opt, loaded_opt) in
            (
                &gravities,
                &mut velocities,
                (&mut decr_velocities).maybe(),
                loadables.maybe(),
                loadeds.maybe(),
            )
                .join()
        {
            if let (None, None) | (Some(_), Some(_)) =
                (loadable_opt, loaded_opt)
            {
                if gravity.enabled {
                    // X
                    if let Some(grav_x) = gravity.x {
                        if grav_x != 0.0 {
                            velocity.x += grav_x * dt;
                            decr_velocity.as_mut().map(|decr| {
                                if grav_x > 0.0 {
                                    decr.should_decrease_x_pos = false;
                                } else if grav_x < 0.0 {
                                    decr.should_decrease_x_neg = false;
                                }
                            });
                        }
                    }

                    // Y
                    if let Some(grav_y) = gravity.y {
                        if grav_y != 0.0 {
                            velocity.y += grav_y * dt;
                            decr_velocity.as_mut().map(|decr| {
                                if grav_y > 0.0 {
                                    decr.should_decrease_y_pos = false;
                                } else if grav_y < 0.0 {
                                    decr.should_decrease_y_neg = false;
                                }
                            });
                        }
                    }
                }
            }
        }
    }
}
