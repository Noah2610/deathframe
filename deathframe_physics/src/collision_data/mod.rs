pub mod prelude {
    pub use super::CollisionData;
    pub use super::CollisionSide;
    pub use super::CollisionState;
}

mod side;
mod state;

pub use side::CollisionSide;
pub use state::CollisionState;

use specs::Entity;
use std::collections::HashMap;

#[derive(Default)]
pub struct CollisionData {
    pub collisions: HashMap<Entity, CollisionState>,
}
