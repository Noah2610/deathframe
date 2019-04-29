use super::system_prelude::*;

/// Gets all entities with `Velocity` and `MaxVelocity`,
/// and caps their velocities if they are above or below a certain limit.
pub struct LimitVelocitiesSystem;

impl<'a> System<'a> for LimitVelocitiesSystem {
    type SystemData =
        (WriteStorage<'a, MaxVelocity>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (mut max_velocities, mut velocities): Self::SystemData) {
        for (max, velocity) in (&mut max_velocities, &mut velocities).join() {
            if max.should_limit_x {
                if let Some(max) = max.x {
                    if velocity.x > 0.0 {
                        velocity.x = velocity.x.min(max)
                    } else if velocity.x < 0.0 {
                        velocity.x = velocity.x.max(-max)
                    }
                }
            } else {
                max.should_limit_x = true;
            }
            if max.should_limit_y {
                if let Some(max) = max.y {
                    if velocity.y > 0.0 {
                        velocity.y = velocity.y.min(max)
                    } else if velocity.y < 0.0 {
                        velocity.y = velocity.y.max(-max)
                    }
                }
            } else {
                max.should_limit_y = true;
            }
        }
    }
}
