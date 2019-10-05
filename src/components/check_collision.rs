use super::component_prelude::*;

/// If an entity with `Collision` also has this marker component `CheckCollision`, then it will
/// actively check for collision against all other entities which have `Collision`.
#[derive(Default, Serialize, Deserialize)]
pub struct CheckCollision;

impl Component for CheckCollision {
    type Storage = NullStorage<Self>;
}

impl Default for CheckCollision {
    fn default() -> Self {
        Self
    }
}
