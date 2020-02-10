use super::system_prelude::*;

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
        let dt = time.delta_seconds();

        for (velocity, base_friction) in
            (&mut velocities, &mut base_frictions).join()
        {
            if base_friction.enabled {
                // v = v * exp(reibung * dt)
            }
        }
    }
}
