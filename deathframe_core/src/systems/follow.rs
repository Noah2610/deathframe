use super::system_prelude::*;

#[derive(Default)]
pub struct FollowSystem;

impl<'a> System<'a> for FollowSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Follow>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, followers, mut transforms): Self::SystemData) {
        let mut followers_join =
            (&entities, &followers).join().collect::<Vec<_>>();
        followers_join.sort_by(|(_, follower_a), (_, follower_b)| {
            follower_a.partial_cmp(follower_b).unwrap()
        });

        for (follower_entity, follower) in followers_join {
            if let Some(followed_pos) =
                transforms.get(follower.to_follow).map(|transform| {
                    let translation = transform.translation();
                    (translation.x, translation.y)
                })
            {
                if let Some(follower_transform) =
                    transforms.get_mut(follower_entity)
                {
                    let new_pos = (
                        followed_pos.0 + follower.offset.0,
                        followed_pos.1 + follower.offset.1,
                    );
                    match follower.only_axis.as_ref() {
                        None => {
                            follower_transform.set_translation_x(new_pos.0);
                            follower_transform.set_translation_y(new_pos.1);
                        }
                        Some(Axis::X) => {
                            follower_transform.set_translation_x(new_pos.0);
                        }
                        Some(Axis::Y) => {
                            follower_transform.set_translation_y(new_pos.1);
                        }
                    }
                }
            }
        }
    }
}
