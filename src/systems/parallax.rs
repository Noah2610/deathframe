use super::system_prelude::*;

/// The `ParallaxSystem` is in charge of managing all parallax backgrounds.
pub struct ParallaxSystem;

impl<'a> System<'a> for ParallaxSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Parallax>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (entities, parallaxes, sizes, mut transforms): Self::SystemData,
    ) {
        // Create a HashMap of entities and their positions, which are followed by all
        // parallax entities. Keys are parallax entities' IDs, values are followed entities data.
        let following_entities = parallax_following_data_for(
            &entities,
            &parallaxes,
            &transforms,
            &sizes,
        );

        // Loop through all parallax entities and actually move them.
        for (parallax_entity, parallax_transform, parallax) in
            (&entities, &mut transforms, &parallaxes).join()
        {
            let parallax_id = parallax_entity.id();
            // Only move them if following entity data was found in the previous step
            if let Some(ParallaxFollowingData {
                id: following_id,
                pos: following_pos,
                size: following_size_opt,
            }) = following_entities.get(&parallax_id)
            {
                // TODO: Textures are not _repeated_.
                //       So it is very possible for the camera to see the border of the texture.
                // Calaculate and apply the new position for the parallax background.
                let new_pos = parallax.calculate_pos_with_following(
                    *following_pos,
                    *following_size_opt,
                );
                // let new_x = following_middle.0 + parallax.offset.0
                //     - following_middle.0 * parallax.speed_mult.0;
                // let new_y = following_middle.1 + parallax.offset.1
                //     - following_middle.1 * parallax.speed_mult.1;
                parallax_transform.set_x(new_pos.0);
                parallax_transform.set_y(new_pos.1);
            }
        }
    }
}
