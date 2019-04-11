use amethyst::ecs::world::Index;

use super::component_prelude::*;

mod defaults {
    use super::Vector;
    pub const BASE_SPEED: Vector = (250.0, 250.0);
    pub const DEADZONE: Vector = (16.0, 16.0);
}

/// Wrapper component for amethyst's `Camera` component.
/// This `Camera` may follow an entity, given by their entity ID.
/// An entity with this component won't do much without it also having
/// an `amethyst::renderer::Camera` component.
pub struct Camera {
    pub(crate) base_speed: Vector,
    pub(crate) deadzone:   Vector,

    /// Follow the entity with this ID.
    pub follow: Option<Index>,
}

impl Camera {
    /// Returns a new `CameraBuilder`
    pub fn new() -> CameraBuilder {
        CameraBuilder::default()
    }
}

pub struct CameraBuilder {
    pub base_speed: Option<Vector>,
    pub deadzone:   Option<Vector>,
    pub follow:     Option<Index>,
}

/// A builder struct for `Camera`.
impl CameraBuilder {
    /// Set the `base_speed`.
    pub fn base_speed(mut self, base_speed: Vector) -> Self {
        self.base_speed = Some(base_speed);
        self
    }

    /// Set the `deadzone`.
    pub fn deadzone(mut self, deadzone: Vector) -> Self {
        self.deadzone = Some(deadzone);
        self
    }

    /// Set the following entity's ID.
    pub fn follow(mut self, follow: Index) -> Self {
        self.follow = Some(follow);
        self
    }

    /// Build the `Camera`.
    pub fn build(self) -> Camera {
        let default = Camera::default();
        let base_speed = self.base_speed.unwrap_or(default.base_speed);
        let deadzone = self.deadzone.unwrap_or(default.deadzone);
        let follow = if self.follow.is_some() {
            self.follow
        } else {
            default.follow
        };
        Camera {
            base_speed,
            deadzone,
            follow,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            base_speed: None,
            deadzone:   None,
            follow:     None,
        }
    }
}

impl Component for Camera {
    type Storage = HashMapStorage<Self>;
}

impl Default for Camera {
    fn default() -> Self {
        use defaults::*;
        Camera {
            base_speed: BASE_SPEED,
            deadzone:   DEADZONE,
            follow:     None,
        }
    }
}
