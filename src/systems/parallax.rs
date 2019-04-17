use std::collections::HashMap;

use super::system_prelude::*;
use crate::geo::Vector;

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
        let following_entities = (&entities, &parallaxes)
            .join()
            .filter_map(|(parallax_entity, parallax)| {
                if let Some(target_id) = parallax.follow {
                    let follow_data_opt =
                        (&entities, &transforms, sizes.maybe())
                            .join()
                            .find_map(|(entity, transform, size_opt)| {
                                let entity_id = entity.id();
                                if target_id == entity_id {
                                    Some((
                                        entity_id,
                                        transform.into(),
                                        size_opt.map(|size| size.into()),
                                    ))
                                } else {
                                    None
                                }
                            });
                    if let Some(follow_data) = follow_data_opt {
                        Some((parallax_entity.id(), follow_data))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<HashMap<Index, (Index, Vector, Option<Vector>)>>();

        // Loop through all parallax entities and actually move them.
        for (parallax_entity, parallax_transform, parallax) in
            (&entities, &mut transforms, &parallaxes).join()
        {
            let parallax_id = parallax_entity.id();
            // Only move them if following entity data was found in the previous step
            if let Some((following_id, following_pos, following_size_opt)) =
                following_entities.get(&parallax_id)
            {
                let parallax_pos = Vector::from(&*parallax_transform);
                let following_middle =
                    if let Some(following_size) = following_size_opt {
                        parallax
                            .follow_anchor
                            .middle_for(following_pos, following_size)
                    } else {
                        *following_pos
                    };

                // TODO: TEMPORARY
                // let calculated_offset = (
                //     (parallax_pos.0 - following_middle.0)
                //         * parallax.speed_mult.0,
                //     (parallax_pos.1 - following_middle.1)
                //         * parallax.speed_mult.1,
                // );
                let new_x = (following_middle.0 + parallax.offset.0)
                    * parallax.speed_mult.0;
                let new_y = (following_middle.1 + parallax.offset.1)
                    * parallax.speed_mult.1;
                parallax_transform.set_x(new_x);
                parallax_transform.set_y(new_y);
            }
        }
    }
}
