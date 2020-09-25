use super::system_prelude::*;
use std::collections::HashMap;
use std::marker::PhantomData;

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
        ReadStorage<'a, SolidPusher>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            mut transform_store,
            mut velocity_store,
            hitbox_store,
            solid_store,
            solid_pusher_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        Self::run_without_collision(
            dt,
            &entities,
            &mut transform_store,
            &mut velocity_store,
            &solid_store,
            &unloaded_store,
        );

        Self::run_with_collision(
            dt,
            &entities,
            &mut transform_store,
            &mut velocity_store,
            &solid_store,
            &solid_pusher_store,
            &hitbox_store,
            &unloaded_store,
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
        transform_store: &mut WriteStorage<Transform>,
        velocity_store: &mut WriteStorage<Velocity>,
        solid_store: &ReadStorage<Solid<C>>,
        unloaded_store: &ReadStorage<Unloaded>,
    ) {
        for (_, transform, velocity, _, _) in (
            entities,
            transform_store,
            velocity_store,
            !solid_store,
            !unloaded_store,
        )
            .join()
        {
            transform.prepend_translation_x(velocity.x * dt);
            transform.prepend_translation_y(velocity.y * dt);
        }
    }

    fn run_with_collision(
        dt: f32,
        entities: &Entities,
        transform_store: &mut WriteStorage<Transform>,
        velocity_store: &mut WriteStorage<Velocity>,
        solid_store: &ReadStorage<Solid<C>>,
        solid_pusher_store: &ReadStorage<SolidPusher>,
        hitbox_store: &ReadStorage<Hitbox>,
        unloaded_store: &ReadStorage<Unloaded>,
    ) {
        // Generate the collision grid.
        let mut collision_grid = gen_collision_grid(
            entities,
            &*transform_store,
            hitbox_store,
            solid_store,
            unloaded_store,
            None,
        );

        // Create entity data hashmap for transforms and velocities.
        // At the end of the function each entity's transform and velocity
        // components are updated with theses values.
        let mut entity_data_map = EntityDataMap::new();
        for (entity, transform, _) in
            (entities, &*transform_store, !unloaded_store).join()
        {
            entity_data_map.insert(entity, EntityData::from(transform));
        }

        for (entity, transform, velocity, _solid, _hitbox, _) in (
            entities,
            &*transform_store,
            velocity_store,
            solid_store,
            hitbox_store,
            !unloaded_store,
        )
            .join()
        {
            move_entity(
                dt,
                &mut collision_grid,
                &mut entity_data_map,
                entity,
                transform,
                velocity,
                entities,
                solid_store,
                hitbox_store,
                solid_pusher_store,
            );
        }

        // Apply changed entity data to respective components.
        for (entity, EntityData { position }) in entity_data_map {
            if let Some(transform) = transform_store.get_mut(entity) {
                transform.set_translation_x(position.x);
                transform.set_translation_y(position.y);
            }
        }
    }
}

fn move_entity<C>(
    dt: f32,
    collision_grid: &mut CollisionGrid<Entity, C, ()>,
    entity_data_map: &mut EntityDataMap,
    entity: Entity,
    transform: &Transform,
    velocity: &mut Velocity,
    entities: &Entities,
    solid_store: &ReadStorage<Solid<C>>,
    hitbox_store: &ReadStorage<Hitbox>,
    pusher_store: &ReadStorage<SolidPusher>,
) where
    C: CollisionTag,
{
    Axis::for_each(|axis| {
        let vel = match axis {
            Axis::X => velocity.x * dt,
            Axis::Y => velocity.y * dt,
        };
        let abs = vel.abs() as usize;
        let sign = if vel != 0.0 { vel.signum() } else { 0.0 };
        let rem = vel % 1.0;

        // Move one pixel at a time
        'pixel_loop: for _ in 0 .. abs {
            if !move_entity_by_one(
                collision_grid,
                entity_data_map,
                entity,
                &axis,
                sign,
                entities,
                solid_store,
                hitbox_store,
                pusher_store,
            ) {
                // Entity did not move, would have been in collision.
                // kill the relevant velocity and break out of the loop.
                velocity.clear(&axis);
                break 'pixel_loop;
            }
        }

        // Try to move by the floating point remainder
        if rem != 0.0 {
            if !move_entity_by_one(
                collision_grid,
                entity_data_map,
                entity,
                &axis,
                rem,
                entities,
                solid_store,
                hitbox_store,
                pusher_store,
            ) {
                // Entity did not move, would have been in collision.
                // kill the relevant velocity.
                velocity.clear(&axis);
            }
        }
    });
}

type DidMoveEntity = bool;

fn move_entity_by_one<C>(
    collision_grid: &mut CollisionGrid<Entity, C, ()>,
    entity_data_map: &mut EntityDataMap,
    entity: Entity,
    axis: &Axis,
    step: f32,
    entities: &Entities,
    solid_store: &ReadStorage<Solid<C>>,
    hitbox_store: &ReadStorage<Hitbox>,
    pusher_store: &ReadStorage<SolidPusher>,
) -> DidMoveEntity
where
    C: CollisionTag,
{
    // Remove entity data entry here, is re-inserted at the end of the function
    let EntityData { mut position } = entity_data_map.remove(&entity).expect(
        "Should have `EntityData` for entity in `move_entity_by_one` function",
    );

    let solid = solid_store
        .get(entity)
        .expect("Entity should have `Solid` in `move_entity_by_one` function");
    let hitbox = hitbox_store
        .get(entity)
        .expect("Entity should have `Hitbox` in `move_entity_by_one` function");
    let pusher_opt = pusher_store.get(entity);

    let new_position = {
        match axis {
            Axis::X => Point::new(position.x + step, position.y),
            Axis::Y => Point::new(position.x, position.y + step),
        }
    };

    let is_pusher = pusher_opt.is_some();

    let (is_position_in_collision, colliding_rects) = {
        let collision_rect = CollisionRect::builder()
            .id(entity.id())
            .tag(solid.tag.clone())
            .rects(
                hitbox
                    .rects
                    .clone()
                    .into_iter()
                    .map(|rect| rect.with_offset(&new_position))
                    .collect(),
            )
            .build()
            .unwrap();
        if is_pusher {
            let colliding: Vec<_> = collision_grid
                .colliding_with(&collision_rect)
                .into_iter()
                .cloned()
                .collect();
            (!colliding.is_empty(), Some(colliding))
        } else {
            (collision_grid.collides_any(&collision_rect), None)
        }
    };

    let mut set_new_position =
        |collision_grid: &mut CollisionGrid<Entity, C, ()>| {
            // Update position
            position.x = new_position.x;
            position.y = new_position.y;
            // Update position in collision_grid
            if let Some(collision_rect) = collision_grid.get_mut(&entity) {
                let new_rect = gen_collision_rect(
                    &entity,
                    &position,
                    hitbox,
                    solid.tag.clone(),
                    &None,
                );
                *collision_rect = new_rect;
            }
        };

    let did_move_entity = if is_position_in_collision {
        if is_pusher {
            if let Some(colliding_rects) = colliding_rects {
                // SolidPusher is in collision, so try to push colliding entities,
                // and move self if they were moved successfully.
                let did_move_colliding_rects =
                    colliding_rects.into_iter().all(|colliding| {
                        move_entity_by_one(
                            collision_grid,
                            entity_data_map,
                            entities.entity(colliding.id),
                            axis,
                            step,
                            entities,
                            solid_store,
                            hitbox_store,
                            pusher_store,
                        )
                    });
                if did_move_colliding_rects {
                    // Move this entity, because all colliding entities were moved
                    set_new_position(collision_grid);
                }
                did_move_colliding_rects
            } else {
                panic!(
                    "`colliding_rects` has to be `Some` for `SolidPusher` \
                     entity that is in collision"
                )
            }
        } else {
            // New position would be in collision,
            false
        }
    } else {
        // New position is NOT in collision, apply position
        set_new_position(collision_grid);
        true
    };

    // Re-insert position and velocity entity data
    entity_data_map.insert(entity, EntityData { position });

    did_move_entity
}

impl<C> Default for MoveEntitiesSystem<C>
where
    C: 'static + CollisionTag,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

type EntityDataMap = HashMap<Entity, EntityData>;

struct EntityData {
    pub position: Point,
}

impl<'a> From<&'a Transform> for EntityData {
    fn from(transform: &'a Transform) -> Self {
        let trans = transform.translation();
        let position = Point::new(trans.x, trans.y);
        Self { position }
    }
}
