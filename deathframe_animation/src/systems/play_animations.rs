use super::system_prelude::*;

/// Handles the playing of animations for entities with `Animation`.
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
    }
}
