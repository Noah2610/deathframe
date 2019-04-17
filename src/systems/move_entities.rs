use std::collections::HashMap;

use super::system_prelude::*;
use crate::geo::prelude::*;

/// This system is responsible for moving all entities with `Transform` and `Velocity`,
/// by manipulating their `Transform` appropriately.
/// It also handles collision with `Solid` entities; Solid entities may not move into each other.
pub struct MoveEntitiesSystem;

impl<'a> System<'a> for MoveEntitiesSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Solid>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Push>,
        ReadStorage<'a, Pushable>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            solids,
            sizes,
            pushers,
            pushables,
            mut transforms,
            mut velocities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        self.run_without_collision(
            dt,
            &solids,
            &mut transforms,
            &mut velocities,
        );

        self.run_with_collision(
            dt,
            &entities,
            &solids,
            &sizes,
            &pushers,
            &pushables,
            &mut transforms,
            &mut velocities,
        );
    }
}

impl<'a> MoveEntitiesSystem {
    fn run_without_collision(
        &self,
        dt: f32,
        solids: &ReadStorage<Solid>,
        transforms: &mut WriteStorage<'a, Transform>,
        velocities: &mut WriteStorage<'a, Velocity>,
    ) {
        for (velocity, transform, _) in (velocities, transforms, !solids).join()
        {
            transform.translate_x(velocity.x * dt);
            transform.translate_y(velocity.y * dt);
        }
    }

    fn run_with_collision(
        &self,
        dt: f32,
        entities: &Entities<'a>,
        solids: &ReadStorage<'a, Solid>,
        sizes: &ReadStorage<'a, Size>,
        pushers: &ReadStorage<'a, Push>,
        pushables: &ReadStorage<'a, Pushable>,
        transforms: &mut WriteStorage<'a, Transform>,
        velocities: &mut WriteStorage<'a, Velocity>,
    ) {
        // Generate CollisionGrid with all solid entities
        // The custom generic `bool` represents if it is pushable or not
        let collision_grid = CollisionGrid::<bool>::from(
            (
                entities,
                &*transforms,
                sizes.maybe(),
                pushables.maybe(),
                solids,
            )
                .join()
                .map(|(entity, transform, size_opt, pushable_opt, _)| {
                    let pos = transform.translation();
                    (
                        entity.id(),
                        (pos.x, pos.y).into(),
                        size_opt.map(|size| (size.w, size.h).into()),
                        pushable_opt.map(|_| true),
                    )
                })
                .collect::<Vec<(Index, Vector, Option<Vector>, Option<bool>)>>(
                ),
        );
        // This HashMap will be filled with entity IDs (keys) and a vector (values), by
        // which they must be moved afterwards.
        let mut translate_pushables = HashMap::new();

        // Now check for collisions for all solid entities, using the generated CollisionGrid
        for (entity, velocity, size_opt, transform, pusher_opt, _) in (
            entities,
            &*velocities,
            sizes.maybe(),
            &mut *transforms,
            pushers.maybe(),
            solids,
        )
            .join()
        {
            let entity_id = entity.id();
            Axis::for_each(|axis| {
                let vel = match axis {
                    Axis::X => velocity.x * dt,
                    Axis::Y => velocity.y * dt,
                };
                let abs = vel.abs() as usize;
                let sign = if vel != 0.0 { vel.signum() } else { 0.0 };
                let rem = vel % 1.0;

                // Try to move by one absolute unit
                for _ in 0 ..= abs {
                    let (collision_rect, new_position) =
                        new_collision_rect_and_position(
                            entity_id, transform, size_opt, &axis, sign,
                        );
                    // Check for collision in newly calculated position
                    let colliding_with =
                        collision_grid.colliding_with(&collision_rect);
                    if colliding_with.is_empty() {
                        // New position would NOT be in collision, apply new position
                        transform.set_x(new_position.0);
                        transform.set_y(new_position.1);
                    } else {
                        // New position would be in collision, break out of loop and don't apply
                        // new position, unless this entity is `Push`, and all colliding entities
                        // are `Pushable`.
                        if pusher_opt.is_some() {
                            if colliding_with
                                .iter()
                                .all(|rect| rect.custom.unwrap_or(false))
                            {
                                // All colliding entities are `Pushable`, therefor push them.
                                // Afterwards, they will really be pushed (transforms manipulated),
                                // for now we will only note, that the do need to be translated.
                                // Also move itself.
                                for coll_with in colliding_with {
                                    let entry = translate_pushables
                                        .entry(coll_with.id)
                                        .or_insert((0.0, 0.0));
                                    //*entry = new_position;
                                    match axis {
                                        Axis::X => entry.0 += sign,
                                        Axis::Y => entry.1 += sign,
                                    }
                                }
                                transform.set_x(new_position.0);
                                transform.set_y(new_position.1);
                            } else {
                                // None of the entities are `Pushable`, so don't apply new position.
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                // Try to move by the floating point remainder
                // Calculate new position
                let (collision_rect, new_position) =
                    new_collision_rect_and_position(
                        entity_id, transform, size_opt, &axis, rem,
                    );
                // Check for collision in newly calculated position
                let colliding_with =
                    collision_grid.colliding_with(&collision_rect);
                if colliding_with.is_empty() {
                    // New position would NOT be in collision, apply new position
                    transform.set_x(new_position.0);
                    transform.set_y(new_position.1);
                } else {
                    // New position would be in collision, check if all collidin entities are pushable.
                    if pusher_opt.is_some() {
                        if colliding_with
                            .iter()
                            .all(|rect| rect.custom.unwrap_or(false))
                        {
                            // All colliding entities are `Pushable`, therefor push them.
                            // Afterwards, they will really be pushed (transforms manipulated),
                            // for now we will only note, that the do need to be translated.
                            for coll_with in colliding_with {
                                let entry = translate_pushables
                                    .entry(coll_with.id)
                                    .or_insert((0.0, 0.0));
                                //*entry = new_position;
                                match axis {
                                    Axis::X => entry.0 += rem,
                                    Axis::Y => entry.1 += rem,
                                }
                            }
                            transform.set_x(new_position.0);
                            transform.set_y(new_position.1);
                        }
                    }
                }
            });
        } // End join loop

        // Push all pushable entities, which need pushing
        // Also kill their velocities, if they have one
        for (id, (x, y)) in translate_pushables {
            for (entity, transform, mut velocity_opt, _) in (
                entities,
                &mut *transforms,
                (&mut *velocities).maybe(),
                pushables,
            )
                .join()
            {
                if entity.id() == id {
                    velocity_opt.as_mut().map(|velocity| {
                        if x != 0.0 {
                            velocity.x = 0.0;
                        }
                        if y != 0.0 {
                            velocity.y = 0.0;
                        }
                    });
                    transform.translate_x(x);
                    transform.translate_y(y);
                }
            }
        }
    }
}

fn new_collision_rect_and_position<T>(
    id: Index,
    transform: &Transform,
    size_opt: Option<&Size>,
    axis: &Axis,
    step: f32,
) -> (CollisionRect<T>, Vector) {
    // Calculate new position
    let pos = transform.translation();
    let new_position = (
        pos.x + if axis.is_x() { step } else { 0.0 },
        pos.y + if axis.is_y() { step } else { 0.0 },
    )
        .into();
    // Create a CollisionRect with new position
    (
        CollisionRect::new(
            id,
            new_position,
            size_opt.map(|size| (size.w, size.h).into()),
        ),
        new_position,
    )
}
