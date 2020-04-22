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

        for (_, velocity, base_friction, loadable_opt, loaded_opt) in (
            &entities,
            &mut velocities,
            &mut base_frictions,
            loadables.maybe(),
            loadeds.maybe(),
        )
            .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
            {
                Axis::for_each(|axis| {
                    let vel = velocity.get(&axis);
                    if vel.abs() > VELOCITY_MARGIN {
                        if let Some(fric) = base_friction.get(&axis) {
                            // Exponential, but may cause side-effects /
                            // problems with frame rate discrepancies:
                            // let reduced_vel = vel - vel * fric * dt;

                            // Exponential, but uncomprehensive configuration (friction value).
                            // The configured value is also exponential,
                            // so small changes can have a big impact.
                            // let reduced_vel = vel * (-fric * dt).exp();

                            // Exponential and easily configured.
                            // The value is configured linearly, so doubling the friction factor
                            // will also half the friction effectiveness.
                            // BUT, the value has to be `1.0` or larger, or weird stuff happens!
                            assert!(
                                fric >= 1.0,
                                "The friction value has to be larger than or \
                                 equal to 1.0"
                            );
                            let reduced_vel =
                                vel * (-(fric.log((1.0_f32).exp())) * dt).exp();

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
}
