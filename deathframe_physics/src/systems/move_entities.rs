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
        ReadStorage<'a, Unloaded>,
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
            unloaded_store,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        Self::run_without_collision(
            dt,
            &entities,
            &mut transforms,
            &mut velocities,
            &solids,
            &unloaded_store,
        );

        Self::run_with_collision(
            dt,
            &entities,
            &mut transforms,
            &mut velocities,
            &solids,
            &hitboxes,
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
        transforms: &mut WriteStorage<Transform>,
        velocities: &mut WriteStorage<Velocity>,
        solids: &ReadStorage<Solid<C>>,
        unloaded_store: &ReadStorage<Unloaded>,
    ) {
        for (_, transform, velocity, _, _) in
            (entities, transforms, velocities, !solids, !unloaded_store).join()
        {
            transform.prepend_translation_x(velocity.x * dt);
            transform.prepend_translation_y(velocity.y * dt);
        }
    }

    fn run_with_collision(
        dt: f32,
        entities: &Entities,
        transforms: &mut WriteStorage<Transform>,
        velocities: &mut WriteStorage<Velocity>,
        solids: &ReadStorage<Solid<C>>,
        hitboxes: &ReadStorage<Hitbox>,
        unloaded_store: &ReadStorage<Unloaded>,
    ) {
        // Generate the collision grid.
        let mut collision_grid = gen_collision_grid(
            entities,
            &*transforms,
            hitboxes,
            solids,
            unloaded_store,
            None,
        );

        for (entity, transform, velocity, solid, hitbox, _) in (
            entities,
            transforms,
            velocities,
            solids,
            hitboxes,
            !unloaded_store,
        )
            .join()
        {
            move_enitity(
                dt,
                &mut collision_grid,
                entity,
                transform,
                velocity,
                solid,
                hitbox,
            );
        }
    }
}

fn move_enitity<C>(
    dt: f32,
    collision_grid: &mut CollisionGrid<Entity, C, ()>,
    entity: Entity,
    transform: &mut Transform,
    velocity: &mut Velocity,
    solid: &Solid<C>,
    hitbox: &Hitbox,
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
                entity,
                transform,
                velocity,
                solid,
                hitbox,
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
                entity,
                transform,
                velocity,
                solid,
                hitbox,
                &axis,
                rem,
            );
        }
    });
}

type DidMoveEntity = bool;

fn move_entity_by_one<C>(
    collision_grid: &mut CollisionGrid<Entity, C, ()>,
    entity: Entity,
    transform: &mut Transform,
    velocity: &mut Velocity,
    solid: &Solid<C>,
    hitbox: &Hitbox,
    axis: &Axis,
    step: f32,
) -> DidMoveEntity
where
    C: CollisionTag,
{
    let new_position = {
        let trans = transform.translation();
        match axis {
            Axis::X => Point::new(trans.x + step, trans.y),
            Axis::Y => Point::new(trans.x, trans.y + step),
        }
    };

    let is_position_in_collision = {
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
        collision_grid.collides_any(&collision_rect)
    };

    if is_position_in_collision {
        // New position would be in collision,
        // kill the relevant velocity and break out of the loop.
        velocity.clear(&axis);
        false
    } else {
        // New position is NOT in collision, apply position
        transform.set_translation_x(new_position.x);
        transform.set_translation_y(new_position.y);
        // Update position in collision_grid
        if let Some(collision_rect) = collision_grid.get_mut(&entity) {
            let new_rect = gen_collision_rect(
                &entity,
                &*transform,
                hitbox,
                solid.tag.clone(),
                &None,
            );
            *collision_rect = new_rect;
        }
        true
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
