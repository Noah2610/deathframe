use super::system_prelude::*;

/// The `ParallaxSystem` is in charge of managing all parallax backgrounds.
pub struct ParallaxSystem;

impl<'a> System<'a> for ParallaxSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Parallax>,
        ReadStorage<'a, ParallaxRepeat>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (
            entities,
            parallaxes,
            parallax_repeats,
            sizes,
            mut transforms,
        ): Self::SystemData,
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
        for (
            parallax_entity,
            parallax_transform,
            parallax,
            parallax_repeat_opt,
        ) in (
            &entities,
            &mut transforms,
            &parallaxes,
            parallax_repeats.maybe(),
        )
            .join()
        {
            let parallax_id = parallax_entity.id();
            // Only move them if following entity data was found in the previous step
            if let Some(ParallaxFollowingData {
                id: _,
                pos: following_pos,
                size: following_size_opt,
            }) = following_entities.get(&parallax_id)
            {
                // TODO: Textures are not _repeated_.
                //       So it is very possible for the camera to see the border of the texture.
                // Calaculate and apply the new position for the parallax background.
                let new_pos = match parallax_repeat_opt {
                    None => parallax.calculate_pos_with_following(
                        *following_pos,
                        *following_size_opt,
                    ),
                    Some(ParallaxRepeat { repeat_x, repeat_y }) => parallax
                        .calculate_pos_with_following_with_repeat(
                            *following_pos,
                            following_size_opt.expect(
                                "ParallaxRepeat cannot be used when following \
                                 an entity without a Size",
                            ),
                            *repeat_x,
                            *repeat_y,
                        ),
                };
                parallax_transform.set_translation_x(new_pos.0);
                parallax_transform.set_translation_y(new_pos.1);
            }
        }
    }
}
