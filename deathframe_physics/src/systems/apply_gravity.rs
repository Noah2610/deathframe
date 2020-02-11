use super::system_prelude::*;

#[derive(Default)]
pub struct ApplyGravitySystem;

impl<'a> System<'a> for ApplyGravitySystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Gravity>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (time, gravities, mut velocities): Self::SystemData) {
        let dt = time.delta_seconds() as f32;

        for (gravity, velocity) in (&gravities, &mut velocities).join() {
            Axis::for_each(|axis| {
                if let Some(grav) = gravity.get(&axis) {
                    if grav != 0.0 {
                        velocity.increase(&axis, grav * dt);
                    }
                }
            });
        }
    }
}
