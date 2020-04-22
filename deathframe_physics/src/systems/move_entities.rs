use std::marker::PhantomData;

use super::system_prelude::*;

/// This system is responsible for moving all entities with `Transform` and `Velocity`,
/// by manipulating their `Transform` appropriately.
/// It also handles collision with `Solid` entities; Solid entities may not move into each other.
pub struct MoveEntitiesSystem<C>(PhantomData<C>)
where
    C: CollisionTag;

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

        Self::run_with_collision(
            dt,
            &entities,
            &mut transforms,
            &mut velocities,
            &solids,
            &hitboxes,
            &loadables,
            &loadeds,
        );
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
        for (_, transform, velocity, _, loadable_opt, loaded_opt) in (
            entities,
            transforms,
            velocities,
            !solids,
            loadables.maybe(),
            loadeds.maybe(),
        )
            .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
            {
                transform.prepend_translation_x(velocity.x * dt);
                transform.prepend_translation_y(velocity.y * dt);
            }
        }
    }

    fn run_with_collision(
        dt: f32,
        entities: &Entities,
        transforms: &mut WriteStorage<Transform>,
        velocities: &mut WriteStorage<Velocity>,
        solids: &ReadStorage<Solid<C>>,
        hitboxes: &ReadStorage<Hitbox>,
        loadables: &ReadStorage<Loadable>,
        loadeds: &ReadStorage<Loaded>,
    ) {
        // Generate the collision grid.
        let mut collision_grid = gen_collision_grid(
            entities,
            &*transforms,
            hitboxes,
            solids,
            loadables,
            loadeds,
            None,
        );

        for (
            entity,
            transform,
            velocity,
            solid,
            hitbox,
            loadable_opt,
            loaded_opt,
        ) in (
            entities,
            transforms,
            velocities,
            solids,
            hitboxes,
            loadables.maybe(),
            loadeds.maybe(),
        )
            .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
            {
                let entity_id = entity.id();
                let solid_tag = &solid.tag;

                let is_position_in_collision =
                    |position: &Point,
                     collision_grid: &CollisionGrid<Entity, C, ()>|
                     -> bool {
                        let base_coll_rect = CollisionRect::builder()
                            .id(entity_id)
                            .tag(solid_tag.clone());
                        // Check for collision with Hitbox
                        hitbox.rects.iter().any(|hitbox_rect| {
                            let coll_rect = base_coll_rect
                                .clone()
                                .rect(hitbox_rect.clone().with_offset(position))
                                .build()
                                .unwrap();
                            collision_grid.collides_any(&coll_rect)
                        })
                    };

                let set_position = |transform: &mut Transform,
                                    position: Point,
                                    collision_grid: &mut CollisionGrid<
                    Entity,
                    C,
                    (),
                >| {
                    transform.set_translation_x(position.x);
                    transform.set_translation_y(position.y);
                    if let Some(rects) = collision_grid.get_mut(&entity) {
                        let new_rects = gen_collision_rects(
                            &entity,
                            &*transform,
                            hitbox,
                            solid_tag.clone(),
                            &None,
                        );
                        *rects = new_rects;
                    }
                };

                Axis::for_each(|axis| {
                    let vel = match axis {
                        Axis::X => velocity.x * dt,
                        Axis::Y => velocity.y * dt,
                    };
                    let abs = vel.abs() as usize;
                    let sign = if vel != 0.0 { vel.signum() } else { 0.0 };
                    let rem = vel % 1.0;

                    let next_position =
                        |transform: &Transform, step: f32| -> Point {
                            let trans = transform.translation();
                            match axis {
                                Axis::X => Point::new(trans.x + step, trans.y),
                                Axis::Y => Point::new(trans.x, trans.y + step),
                            }
                        };

                    // Move one pixel at a time
                    'pixel_loop: for _ in 0 .. abs {
                        let new_position = next_position(&transform, sign);
                        if is_position_in_collision(
                            &new_position,
                            &collision_grid,
                        ) {
                            // New position would be in collision,
                            // kill the relevant velocity and break out of the loop.
                            velocity.clear(&axis);
                            break 'pixel_loop;
                        } else {
                            // New position is NOT in collision, apply position
                            set_position(
                                transform,
                                new_position,
                                &mut collision_grid,
                            );
                        }
                    }

                    // Try to move by the floating point remainder
                    if rem != 0.0 {
                        let new_position = next_position(&transform, rem);
                        if is_position_in_collision(
                            &new_position,
                            &collision_grid,
                        ) {
                            velocity.clear(&axis);
                        } else {
                            set_position(
                                transform,
                                new_position,
                                &mut collision_grid,
                            );
                        }
                    }
                });
            }
        }
    }
}

impl<C> Default for MoveEntitiesSystem<C>
where
    C: 'static + CollisionTag,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
