use super::system_prelude::*;

#[derive(Default)]
pub struct ApplyGravitySystem;

impl<'a> System<'a> for ApplyGravitySystem {
    type SystemData = (
        Read<'a, Time>,
        Entities<'a>,
        ReadStorage<'a, Gravity>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (time, entities, gravities, mut velocities, unloaded_store): Self::SystemData,
    ) {
        let dt = time.delta_seconds() as f32;

        for (_, gravity, velocity, _) in
            (&entities, &gravities, &mut velocities, !&unloaded_store).join()
        {
            Axis::for_each(|axis| {
                if gravity.enabled.by_axis(&axis) {
                    if let Some(grav) = gravity.get(&axis) {
                        if grav != 0.0 {
                            velocity.increase(&axis, grav * dt);
                        }
                    }
                }
            });
        }
    }
}
