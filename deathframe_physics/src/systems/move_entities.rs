//! TODO: Rewrite this...

use std::collections::HashMap;
use std::marker::PhantomData;

use super::system_prelude::*;

/// This system is responsible for moving all entities with `Transform` and `Velocity`,
/// by manipulating their `Transform` appropriately.
/// It also handles collision with `Solid` entities; Solid entities may not move into each other.
#[derive(Default)]
pub struct MoveEntitiesSystem<T>(PhantomData<T>)
where
    T: CollisionTag;

impl<'a, C> System<'a> for MoveEntitiesSystem<C>
where
    C: 'static + CollisionTag,
{
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Hitbox>,
        ReadStorage<'a, Solid<C>>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            mut transforms,
            mut velocities,
            hitboxes,
            solids,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        Self::run_without_collision(
            dt,
            &entities,
            &mut transforms,
            &mut velocities,
            &solids,
            &loadables,
            &loadeds,
        );

        // Self::run_with_collision(
        //     dt,
        //     &entities,
        //     &solids,
        //     &sizes,
        //     &pushers,
        //     &pushables,
        //     &loadables,
        //     &loadeds,
        //     &mut transforms,
        //     &mut velocities,
        // );
    }
}

impl<C> MoveEntitiesSystem<C>
where
    C: 'static + CollisionTag,
{
    fn run_without_collision(
        dt: f32,
        entities: &Entities,
        transforms: &mut WriteStorage<Transform>,
        velocities: &mut WriteStorage<Velocity>,
        solids: &ReadStorage<Solid<C>>,
        loadables: &ReadStorage<Loadable>,
        loadeds: &ReadStorage<Loaded>,
    ) {
        for (entity, transform, velocity, _) in
            (entities, transforms, velocities, !solids).join()
        {
            if is_entity_loaded(entity, loadables, loadeds) {
                transform.prepend_translation_x(velocity.x * dt);
                transform.prepend_translation_y(velocity.y * dt);
            }
        }
    }

    //fn run_with_collision(
    //    dt: f32,
    //    entities: &Entities<'a>,
    //    solids: &ReadStorage<'a, Solid<T>>,
    //    sizes: &ReadStorage<'a, Size>,
    //    pushers: &ReadStorage<'a, Push>,
    //    pushables: &ReadStorage<'a, Pushable>,
    //    loadables: &ReadStorage<Loadable>,
    //    loadeds: &ReadStorage<Loaded>,
    //    transforms: &mut WriteStorage<'a, Transform>,
    //    velocities: &mut WriteStorage<'a, Velocity>,
    //) {
    //    const ERRMSG_ID: &str = "`CollisionRect` should have an `id` here";

    //    // Generate CollisionGrid with all solid entities
    //    // The custom generic `bool` represents if it is pushable or not
    //    let collision_grid = CollisionGrid::<T, bool>::from(
    //        (
    //            entities,
    //            &*transforms,
    //            sizes.maybe(),
    //            solids,
    //            pushables.maybe(),
    //            loadables.maybe(),
    //            loadeds.maybe(),
    //        )
    //            .join()
    //            .filter_map(
    //                |(
    //                    entity,
    //                    transform,
    //                    size_opt,
    //                    solid,
    //                    pushable_opt,
    //                    loadable_opt,
    //                    loaded_opt,
    //                )| {
    //                    if let (None, None) | (Some(_), Some(_)) =
    //                        (loadable_opt, loaded_opt)
    //                    {
    //                        let pos = transform.translation();
    //                        Some((
    //                            entity.id(),
    //                            (pos.x, pos.y).into(),
    //                            size_opt.map(|size| (size.w, size.h).into()),
    //                            solid.tag.clone(),
    //                            pushable_opt.map(|_| true),
    //                        ))
    //                    } else {
    //                        None
    //                    }
    //                },
    //            )
    //            .collect::<Vec<(
    //                Index,
    //                Vector,
    //                Option<Vector>,
    //                T,
    //                Option<bool>,
    //            )>>(),
    //    );
    //    // This HashMap will be filled with entity IDs (keys) and a vector (values), by
    //    // which they must be moved afterwards.
    //    let mut translate_pushables: HashMap<Index, (f32, f32)> =
    //        HashMap::new();

    //    // Now check for collisions for all solid entities, using the generated CollisionGrid
    //    for (
    //        entity,
    //        velocity,
    //        size_opt,
    //        solid,
    //        transform,
    //        pusher_opt,
    //        loadable_opt,
    //        loaded_opt,
    //    ) in (
    //        entities,
    //        &*velocities,
    //        sizes.maybe(),
    //        solids,
    //        &mut *transforms,
    //        pushers.maybe(),
    //        loadables.maybe(),
    //        loadeds.maybe(),
    //    )
    //        .join()
    //    {
    //        if let (None, None) | (Some(_), Some(_)) =
    //            (loadable_opt, loaded_opt)
    //        {
    //            let entity_id = entity.id();
    //            Axis::for_each(|axis| {
    //                let vel = match axis {
    //                    Axis::X => velocity.x * dt,
    //                    Axis::Y => velocity.y * dt,
    //                };
    //                let abs = vel.abs() as usize;
    //                let sign = if vel != 0.0 { vel.signum() } else { 0.0 };
    //                let rem = vel % 1.0;

    //                // Try to move by one absolute unit
    //                for _ in 0 .. abs {
    //                    let (collision_rect, new_position) =
    //                        new_collision_rect_and_position(
    //                            entity_id,
    //                            transform,
    //                            size_opt,
    //                            solid.tag.clone(),
    //                            &axis,
    //                            sign,
    //                        );
    //                    // Check for collision in newly calculated position
    //                    let colliding_with =
    //                        collision_grid.colliding_with(&collision_rect);
    //                    if colliding_with.is_empty() {
    //                        // New position would NOT be in collision, apply new position
    //                        transform.set_translation_x(new_position.0);
    //                        transform.set_translation_y(new_position.1);
    //                    } else {
    //                        // New position would be in collision, break out of loop and don't apply
    //                        // new position, unless this entity is `Push`, and all colliding entities
    //                        // are `Pushable`.
    //                        if pusher_opt.is_some() {
    //                            if colliding_with
    //                                .iter()
    //                                .all(|rect| rect.custom.unwrap_or(false))
    //                            {
    //                                // All colliding entities are `Pushable`, therefor push them.
    //                                // Afterwards, they will really be pushed (transforms manipulated),
    //                                // for now we will only note, that the do need to be translated.
    //                                // Also move itself.
    //                                for coll_with in colliding_with {
    //                                    let entry = translate_pushables
    //                                        .entry(
    //                                            coll_with.id.expect(ERRMSG_ID),
    //                                        )
    //                                        .or_insert((0.0, 0.0));
    //                                    //*entry = new_position;
    //                                    match axis {
    //                                        Axis::X => entry.0 += sign,
    //                                        Axis::Y => entry.1 += sign,
    //                                    }
    //                                }
    //                                transform.set_translation_x(new_position.0);
    //                                transform.set_translation_y(new_position.1);
    //                            } else {
    //                                // None of the entities are `Pushable`, so don't apply new position.
    //                                break;
    //                            }
    //                        } else {
    //                            break;
    //                        }
    //                    }
    //                }
    //                // Try to move by the floating point remainder
    //                // Calculate new position
    //                let (collision_rect, new_position) =
    //                    new_collision_rect_and_position(
    //                        entity_id,
    //                        transform,
    //                        size_opt,
    //                        solid.tag.clone(),
    //                        &axis,
    //                        rem,
    //                    );
    //                // Check for collision in newly calculated position
    //                let colliding_with =
    //                    collision_grid.colliding_with(&collision_rect);
    //                if colliding_with.is_empty() {
    //                    // New position would NOT be in collision, apply new position
    //                    transform.set_translation_x(new_position.0);
    //                    transform.set_translation_y(new_position.1);
    //                } else {
    //                    // New position would be in collision, check if all collidin entities are pushable.
    //                    if pusher_opt.is_some() {
    //                        if colliding_with
    //                            .iter()
    //                            .all(|rect| rect.custom.unwrap_or(false))
    //                        {
    //                            // All colliding entities are `Pushable`, therefor push them.
    //                            // Afterwards, they will really be pushed (transforms manipulated),
    //                            // for now we will only note, that the do need to be translated.
    //                            for coll_with in colliding_with {
    //                                let entry = translate_pushables
    //                                    .entry(coll_with.id.expect(ERRMSG_ID))
    //                                    .or_insert((0.0, 0.0));
    //                                //*entry = new_position;
    //                                match axis {
    //                                    Axis::X => entry.0 += rem,
    //                                    Axis::Y => entry.1 += rem,
    //                                }
    //                            }
    //                            transform.set_translation_x(new_position.0);
    //                            transform.set_translation_y(new_position.1);
    //                        }
    //                    }
    //                }
    //            });
    //        }
    //    } // End join loop

    //    // Push all pushable entities, which need pushing
    //    // Also kill their velocities, if they have one
    //    for (id, (x, y)) in translate_pushables {
    //        for (entity, transform, mut velocity_opt, _) in (
    //            entities,
    //            &mut *transforms,
    //            (&mut *velocities).maybe(),
    //            pushables,
    //        )
    //            .join()
    //        {
    //            if entity.id() == id {
    //                velocity_opt.as_mut().map(|velocity| {
    //                    if x != 0.0 {
    //                        velocity.x = 0.0;
    //                    }
    //                    if y != 0.0 {
    //                        velocity.y = 0.0;
    //                    }
    //                });
    //                transform.prepend_translation_x(x);
    //                transform.prepend_translation_y(y);
    //            }
    //        }
    //    }
    //}
}

// fn new_collision_rect_and_position<T, T>(
//     id: Index,
//     transform: &Transform,
//     size_opt: Option<&Size>,
//     tag: T,
//     axis: &Axis,
//     step: f32,
// ) -> (CollisionRect<T, T>, Vector)
// where
//     T: CollisionTag,
// {
//     // Calculate new position
//     let pos = transform.translation();
//     let new_position = (
//         pos.x + if axis.is_x() { step } else { 0.0 },
//         pos.y + if axis.is_y() { step } else { 0.0 },
//     )
//         .into();
//     // Create a CollisionRect with new position
//     (
//         CollisionRectBuilder::default()
//             .id(id)
//             .with_pos_and_maybe_size(
//                 new_position,
//                 size_opt.map(|size| (size.w, size.h).into()),
//             )
//             .tag(tag)
//             .build(),
//         new_position,
//     )
// }
