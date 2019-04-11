use amethyst::ecs::world::Index;
use amethyst::renderer::{Camera as AmethystCamera, Projection};

use super::component_prelude::*;

mod defaults {
    use super::Vector;
    pub const SIZE: Vector = (100.0, 100.0);
    pub const BASE_SPEED: Vector = (250.0, 250.0);
    pub const DEADZONE: Vector = (16.0, 16.0);
}

/// Wrapper component for amethyst's `Camera` component.
/// This `Camera` may follow an entity, given by their entity ID.
pub struct Camera {
    pub(crate) camera:     AmethystCamera,
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
    pub camera:     Option<AmethystCamera>,
    pub base_speed: Option<Vector>,
    pub deadzone:   Option<Vector>,
    pub follow:     Option<Index>,
}

/// A builder struct for `Camera`.
impl CameraBuilder {
    /// Set the amethyst camera.
    pub fn camera(mut self, camera: AmethystCamera) -> Self {
        self.camera = Some(camera);
        self
    }

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
        let camera = self.camera.unwrap_or(default.camera);
        let base_speed = self.base_speed.unwrap_or(default.base_speed);
        let deadzone = self.deadzone.unwrap_or(default.deadzone);
        let follow = if self.follow.is_some() {
            self.follow
        } else {
            default.follow
        };
        Camera {
            camera,
            base_speed,
            deadzone,
            follow,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            camera:     None,
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
            camera:     AmethystCamera::from(Projection::orthographic(
                0.0,    // left
                SIZE.0, // right
                0.0,    // bottom
                SIZE.1, // top
            )),
            base_speed: BASE_SPEED,
            deadzone:   DEADZONE,
            follow:     None,
        }
    }
}
