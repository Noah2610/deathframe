pub mod prelude {
    pub use super::lifecycle::Lifecycle;
    pub use super::lifecycle_state::LifecycleState;
}

mod lifecycle;
mod lifecycle_state;

use super::component_prelude;
