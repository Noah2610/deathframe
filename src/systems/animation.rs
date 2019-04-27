//! TODO: Documentation

use std::time::{Duration, Instant};

use super::system_prelude::*;

pub struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, SpriteRender>,
    );

    fn run(
        &mut self,
        (entities, mut animations, mut sprite_renders): Self::SystemData,
    ) {
        let now = Instant::now();

        for (entity, animation) in (&entities, &mut animations).join() {
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
                    .insert(entity, animation.current_sprite_render().clone());

                animation.last_sprite_switch_at = now;
            }
        }
    }
}
