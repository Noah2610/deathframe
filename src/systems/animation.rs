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
            if now - animation.last_sprite_switch_at
                >= Duration::from_millis(animation.current_delay_ms())
            {
                // Next SpriteRender
                let max_index = animation.sprite_renders.len();
                animation.index += 1;

                if animation.index >= max_index {
                    animation.index = 0;
                }

                sprite_renders
                    .insert(entity, animation.current_sprite_render().clone())
                    .unwrap();

                animation.last_sprite_switch_at = now;
            }
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
            if let Some(animation) = &mut animations_container.current {
                if now - animation.last_sprite_switch_at
                    >= Duration::from_millis(animation.current_delay_ms())
                {
                    // Next SpriteRender
                    let max_index = animation.sprite_renders.len();
                    animation.index += 1;

                    if animation.index >= max_index {
                        animation.index = 0;
                    }

                    sprite_renders
                        .insert(
                            entity,
                            animation.current_sprite_render().clone(),
                        )
                        .unwrap();

                    animation.last_sprite_switch_at = now;
                }
            }
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
