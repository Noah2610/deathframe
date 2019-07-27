use super::system_prelude::*;
use amethyst::ecs::storage::MaskedStorage;
use std::collections::HashMap;
use std::ops::Deref;

pub struct ParallaxFollowingData {
    pub id:   Index,
    pub pos:  Vector,
    pub size: Option<Vector>,
}

pub type ParallaxesFollowingData = HashMap<Index, ParallaxFollowingData>;

pub fn parallax_following_data_for<DP, DT, DS>(
    entities: &Entities,
    parallaxes: &Storage<Parallax, DP>,
    transforms: &Storage<Transform, DT>,
    sizes: &Storage<Size, DS>,
) -> ParallaxesFollowingData
where
    DP: Deref<Target = MaskedStorage<Parallax>>,
    DT: Deref<Target = MaskedStorage<Transform>>,
    DS: Deref<Target = MaskedStorage<Size>>,
{
    (entities, parallaxes)
        .join()
        .filter_map(|(parallax_entity, parallax)| {
            if let Some(target_id) = parallax.follow {
                let follow_data_opt = (entities, transforms, sizes.maybe())
                    .join()
                    .find_map(|(entity, transform, size_opt)| {
                        let entity_id = entity.id();
                        if target_id == entity_id {
                            Some(ParallaxFollowingData {
                                id:   entity_id,
                                pos:  transform.into(),
                                size: size_opt.map(|size| size.into()),
                            })
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
        .collect::<ParallaxesFollowingData>()
}