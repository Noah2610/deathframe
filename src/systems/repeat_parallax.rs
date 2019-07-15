use super::system_prelude::*;

/// This system is responsible for repeating/tiling `Parallax` components with `ParallaxRepeat`.
/// It tiles them either _horizontally_ or _vertically_.
/// It creates a new / removes a parallax entity depending on its position relative to the entity
/// it is following.
#[derive(Default)]
pub struct RepeatParallaxSystem;

impl<'a> System<'a> for RepeatParallaxSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, Parallax>,
        WriteStorage<'a, ParallaxRepeat>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut sizes,
            mut parallaxes,
            mut parallax_repeats,
        ): Self::SystemData,
    ) {

    }
}
