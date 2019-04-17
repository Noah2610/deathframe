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
#[derive(Deserialize)]
pub struct Camera {
    #[serde(default)]
    pub(crate) base_speed: Vector,
    #[serde(default)]
    pub(crate) deadzone: Vector,

    /// Follow the entity with this ID.
    #[serde(default)]
    pub follow: Option<Index>,
}

impl Camera {
    /// Returns a new `CameraBuilder`
    pub fn new() -> CameraBuilder {
        CameraBuilder::default()
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

/// A builder struct for `Camera`.
pub struct CameraBuilder {
    base_speed: Vector,
    deadzone:   Vector,
    follow:     Option<Index>,
}

impl CameraBuilder {
    /// Set the `base_speed`.
    pub fn base_speed(mut self, base_speed: Vector) -> Self {
        self.base_speed = base_speed;
        self
    }

    /// Set the `deadzone`.
    pub fn deadzone(mut self, deadzone: Vector) -> Self {
        self.deadzone = deadzone;
        self
    }

    /// Set the following entity's ID.
    pub fn follow(mut self, follow: Index) -> Self {
        self.follow = Some(follow);
        self
    }

    /// Build the `Camera`.
    pub fn build(self) -> Camera {
        Camera {
            base_speed: self.base_speed,
            deadzone:   self.deadzone,
            follow:     self.follow,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        let Camera {
            base_speed,
            deadzone,
            follow,
        } = Camera::default();
        CameraBuilder {
            base_speed,
            deadzone,
            follow,
        }
    }
}
