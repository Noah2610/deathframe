use std::collections::HashMap;

use super::system_prelude::*;

/// The `ParallaxSystem` is in charge of managing all parallax backgrounds.
pub struct ParallaxSystem;

impl<'a> System<'a> for ParallaxSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Parallax>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (entities, parallaxes, mut transforms): Self::SystemData,
    ) {
        // Create a HashMap of entities and their positions, which are followed by all
        // parallax entities. Keys are parallax entities' IDs, values are followed entities data.
        let mut following_entities = HashMap::new();
        for (parallax_entity, parallax) in (&entities, &parallaxes).join() {
            for (entity, transform) in (&entities, &transforms).join() {
                if let Some(following_id) = parallax.follow {
                    let pos = {
                        let translation = transform.translation();
                        (translation.x, translation.y)
                    };
                    following_entities.insert(following_id, (entity.id(),));
                }
            }
        }
    }
}
