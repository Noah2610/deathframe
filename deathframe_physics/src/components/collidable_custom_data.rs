use super::component_prelude::*;
use crate::collision::data::prelude::CollisionCustomData;

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct CollidableCustomData(pub Box<dyn CollisionCustomData>);
