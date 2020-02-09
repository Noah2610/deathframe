pub mod prelude {
    pub use super::move_entities::MoveEntitiesSystem;
    pub use super::update_collisions::UpdateCollisionsSystem;
}

mod system_prelude {
    pub use crate::collision::prelude::*;
    pub use crate::components::prelude::*;
    pub use core::geo::prelude::*;
    pub use core::systems::system_prelude::*;
}

mod move_entities;
mod update_collisions;
