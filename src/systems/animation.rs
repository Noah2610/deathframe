//! TODO: Documentation

use std::time::{Duration, Instant};

use super::system_prelude::*;

pub struct AnimationSystem;

impl AnimationSystem {
    fn run_with_animation<'a>(
        &self,
        now: Instant,
        entities: &Entities<'a>,
        animations: &mut WriteStorage<'a, Animation>,
        sprite_renders: &mut WriteStorage<'a, SpriteRender>,
    ) {
        for (entity, animation) in (entities, animations).join() {
            self.run_for_animation(
                now,
                entity,
                animation,
                sprite_renders,
                true,
            );
        }
    }

    fn run_with_animations_container<'a>(
        &self,
        now: Instant,
        entities: &Entities<'a>,
        animations_containers: &mut WriteStorage<'a, AnimationsContainer>,
        sprite_renders: &mut WriteStorage<'a, SpriteRender>,
    ) {
        for (entity, animations_container) in
            (entities, animations_containers).join()
        {
            if let Some((_, animation)) = &mut animations_container.play_once {
                self.run_for_animation(
                    now,
                    entity,
                    animation,
                    sprite_renders,
                    false,
                );
                if animation.has_played() {
                    animations_container.play_once = None;
                }
            } else if let Some((_, animation)) =
                &mut animations_container.current
            {
                self.run_for_animation(
                    now,
                    entity,
                    animation,
                    sprite_renders,
                    true,
                );
            }
        }
    }

    fn run_for_animation<'a>(
        &self,
        now: Instant,
        entity: Entity,
        animation: &mut Animation,
        sprite_renders: &mut WriteStorage<'a, SpriteRender>,
        should_loop: bool,
    ) {
        if animation.switch_now {
            animation.switch_now = false;
            sprite_renders
                .insert(entity, animation.current_sprite_render().clone())
                .unwrap();
            animation.last_sprite_switch_at = now;
        } else if now - animation.last_sprite_switch_at
            >= Duration::from_millis(animation.current_delay_ms())
        {
            // Next SpriteRender
            let max_index = animation.sprite_renders.len();
            animation.index += 1;

            if animation.index >= max_index {
                // Loop animation
                animation.index = 0;
                animation.played += 1;
            }

            if should_loop || (!should_loop && !animation.has_played()) {
                sprite_renders
                    .insert(entity, animation.current_sprite_render().clone())
                    .unwrap();
            }

            animation.last_sprite_switch_at = now;
        }
    }
}

impl<'a> System<'a> for AnimationSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, AnimationsContainer>,
        WriteStorage<'a, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut animations,
            mut animations_containers,
            mut sprite_renders,
        ): Self::SystemData,
    ) {
        let now = Instant::now();

        self.run_with_animation(
            now,
            &entities,
            &mut animations,
            &mut sprite_renders,
        );

        self.run_with_animations_container(
            now,
            &entities,
            &mut animations_containers,
            &mut sprite_renders,
        );
    }
}
