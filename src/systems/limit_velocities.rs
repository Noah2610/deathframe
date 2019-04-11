use super::system_prelude::*;

/// Gets all entities with `Velocity` and `MaxVelocity`,
/// and caps their velocities if they are above or below a certain limit.
pub struct LimitVelocitiesSystem;

impl<'a> System<'a> for LimitVelocitiesSystem {
    type SystemData =
        (ReadStorage<'a, MaxVelocity>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (max_velocities, mut velocities): Self::SystemData) {
        for (max, velocity) in (&max_velocities, &mut velocities).join() {
            if let Some(max) = max.x {
                if velocity.x > 0.0 {
                    velocity.x = velocity.x.min(max)
                } else if velocity.x < 0.0 {
                    velocity.x = velocity.x.max(-max)
                }
            }
            if let Some(max) = max.y {
                if velocity.y > 0.0 {
                    velocity.y = velocity.y.min(max)
                } else if velocity.y < 0.0 {
                    velocity.y = velocity.y.max(-max)
                }
            }
        }
    }
}
