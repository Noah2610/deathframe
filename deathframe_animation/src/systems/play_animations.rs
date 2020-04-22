use super::system_prelude::*;

/// Handles the playing of animations for entities with `Animation`.
#[derive(Default)]
pub struct PlayAnimationsSystem;

impl<'a> System<'a> for PlayAnimationsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, SpriteRender>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut animations,
            mut sprite_renders,
            loadables,
            loadeds,
        ): Self::SystemData,
    ) {
        for (_, animation, sprite_render, loadable_opt, loaded_opt) in (
            &entities,
            &mut animations,
            &mut sprite_renders,
            loadables.maybe(),
            loadeds.maybe(),
        )
            .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
            {
                animation.update();
                if let Some(sprite_id) = animation.current_sprite_id() {
                    if sprite_id != sprite_render.sprite_number {
                        sprite_render.sprite_number = sprite_id;
                    }
                }
            }
        }
    }
}
