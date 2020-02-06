mod side;
mod state;

pub use side::CollisionSide;
pub use state::CollisionState;

use specs::Entity;
use std::collections::HashMap;

pub struct CollisionData {
    pub collisions: HashMap<Entity, CollisionState>,
}
