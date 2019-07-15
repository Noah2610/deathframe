use super::system_prelude::*;

/// Amount of pixel padding for creating new / deleting existing parallax entities.
const PADDING: f32 = 16.0;

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
        let following_entities = parallax_following_data_for(
            &entities,
            &parallaxes,
            &transforms,
            &sizes,
        );

        for (
            parallax_entity,
            parallax_transform,
            parallax_size,
            parallax,
            parallax_repeat,
        ) in (
            &entities,
            &transforms,
            &sizes,
            &parallaxes,
            &parallax_repeats,
        )
            .join()
        {
            let parallax_id = parallax_entity.id();
            if let Some(ParallaxFollowingData {
                id: following_id,
                pos: following_pos,
                size: following_size_opt,
            }) = following_entities.get(&parallax_id)
            {
                let parallax_rect = Rect::new()
                    .with_pos_and_size(
                        parallax_transform.into(),
                        parallax_size.into(),
                    )
                    .build();
                let following_rect = Rect::new()
                    .with_pos_and_maybe_size(
                        following_pos.clone(),
                        following_size_opt.clone(),
                    )
                    .build();

                let collision_grid = CollisionGrid::<(), ()>::new(vec![
                    CollisionRect::new()
                        .id(parallax_id)
                        .rect(parallax_rect.clone())
                        .build(),
                    CollisionRect::new()
                        .id(*following_id)
                        .rect(following_rect.clone())
                        .build(),
                ]);

                if parallax_repeat.repeat_x {
                    // Does parallax collide with following?
                    // If not, then we can remove the parallax.
                    // TODO: This can cause many problems...

                    // Create new parallax to the _right_?
                    // Is right side of parallax smaller than right side of following?
                    if parallax_rect.right > following_rect.left
                        && parallax_rect.right - PADDING < following_rect.right
                    {
                        // TODO: CREATE NEW TO THE RIGHT
                    } else {

                    }
                    // Create new parallax to the _left_?
                    // Is left side of parallax larger than left side of following?
                    if parallax_rect.left + PADDING > following_rect.left {
                        // TODO: CREATE NEW TO THE RIGHT
                    }
                }
            }
        }
    }
}
