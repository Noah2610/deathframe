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
        // Will be filled when moving entities later,
        // and at the end of the function each entity's transform and velocity
        // components are updated with theses values.
        let mut entity_data_map = EntityDataMap::new();

        for (entity, transform, velocity, solid, hitbox, pusher_opt, _) in (
            entities,
            &*transform_store,
            &*velocity_store,
            solid_store,
            hitbox_store,
            solid_pusher_store.maybe(),
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
                solid,
                hitbox,
                pusher_opt,
            );
        }

        // Apply changed entity data to respective components.
        for (entity, EntityData { position, velocity }) in entity_data_map {
            if let Some(transform) = transform_store.get_mut(entity) {
                transform.set_translation_x(position.x);
                transform.set_translation_y(position.y);
            }
            if let Some(velocity_comp) = velocity_store.get_mut(entity) {
                velocity_comp.x = velocity.x;
                velocity_comp.y = velocity.y;
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
    velocity: &Velocity,
    solid: &Solid<C>,
    hitbox: &Hitbox,
    pusher_opt: Option<&SolidPusher>,
) where
    C: CollisionTag,
{
    entity_data_map.insert(entity, EntityData::from((transform, velocity)));

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
                solid,
                hitbox,
                pusher_opt,
                &axis,
                sign,
            ) {
                // Entity did not move, would have been in collision.
                break 'pixel_loop;
            }
        }

        // Try to move by the floating point remainder
        if rem != 0.0 {
            let _ = move_entity_by_one(
                collision_grid,
                entity_data_map,
                entity,
                solid,
                hitbox,
                pusher_opt,
                &axis,
                rem,
            );
        }
    });
}

type DidMoveEntity = bool;

fn move_entity_by_one<C>(
    collision_grid: &mut CollisionGrid<Entity, C, ()>,
    entity_data_map: &mut EntityDataMap,
    entity: Entity,
    solid: &Solid<C>,
    hitbox: &Hitbox,
    pusher_opt: Option<&SolidPusher>,
    axis: &Axis,
    step: f32,
) -> DidMoveEntity
where
    C: CollisionTag,
{
    // Remove entity data entry here, is re-inserted at the end of the function
    let EntityData {
        mut position,
        mut velocity,
    } = entity_data_map.remove(&entity).expect(
        "Should have `EntityData` for entity in `move_entity_by_one` function",
    );

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
            let colliding = collision_grid.colliding_with(&collision_rect);
            (!colliding.is_empty(), Some(colliding))
        } else {
            (collision_grid.collides_any(&collision_rect), None)
        }
    };

    let did_move_entity = if is_position_in_collision {
        if is_pusher {
            if let Some(colliding_rects) = colliding_rects {
                // SolidPusher is in collision, so try to push colliding entities,
                // and move self if they were moved successfully.
                // TODO
                let moved_colliding_rects = colliding_rects
                    .into_iter()
                    .all(|colliding| unimplemented!());
                unimplemented!()
            } else {
                panic!(
                    "`colliding_rects` has to be `Some` for `SolidPusher` \
                     entity that is in collision"
                )
            }
        } else {
            // New position would be in collision,
            // kill the relevant velocity and break out of the loop.
            match axis {
                Axis::X => velocity.x = 0.0,
                Axis::Y => velocity.y = 0.0,
            }
            false
        }
    } else {
        // New position is NOT in collision, apply position
        // transform.set_translation_x(new_position.x);
        // transform.set_translation_y(new_position.y);
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
        true
    };

    // Re-insert position and velocity entity data
    entity_data_map.insert(entity, EntityData { position, velocity });

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
    pub velocity: Vector,
}

impl<'a> From<(&'a Transform, &'a Velocity)> for EntityData {
    fn from((transform, velocity): (&'a Transform, &'a Velocity)) -> Self {
        let trans = transform.translation();
        let position = Point::new(trans.x, trans.y);
        let velocity = Vector::new(velocity.x, velocity.y);
        Self { position, velocity }
    }
}
