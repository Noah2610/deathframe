use super::component_prelude::*;

/// `Pushable` can be pushed by other `Push` entities.
/// Much jank, don't use (or use with caution).
#[derive(Serialize, Deserialize)]
pub struct Pushable;

impl Component for Pushable {
    type Storage = NullStorage<Self>;
}

impl Default for Pushable {
    fn default() -> Self {
        Self
    }
}
