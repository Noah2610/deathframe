use amethyst::ecs::world::Index;

use super::component_prelude::*;
use crate::geo::Vector;

/// A parallax background. This background has an offset and it can follow another entity,
/// similar to the camera. It is meant to be used with more parallax backgrounds.
/// The speed multiplier determines at what speed the image moves, relative to the following
/// entity. (Does not use velocity.)
pub struct Parallax {
    pub offset:     Vector,
    pub speed_mult: Vector,
    pub follow:     Option<Index>,
}

impl Parallax {
    pub fn new() -> ParallaxBuilder {
        ParallaxBuilder::default()
    }
}

impl Component for Parallax {
    type Storage = HashMapStorage<Self>;
}

impl Default for Parallax {
    fn default() -> Self {
        Self {
            offset:     (0.0, 0.0),
            speed_mult: (0.5, 0.5),
            follow:     None,
        }
    }
}

/// A builder struct for `Parallax`.
pub struct ParallaxBuilder {
    offset:     Vector,
    speed_mult: Vector,
    follow:     Option<Index>,
}

impl ParallaxBuilder {
    /// Set the `offset`.
    pub fn offset(mut self, offset: Vector) -> Self {
        self.offset = offset;
        self
    }

    /// Set the `speed_mult`.
    pub fn speed_mult(mut self, speed_mult: Vector) -> Self {
        self.speed_mult = speed_mult;
        self
    }

    /// Set the entity ID to follow.
    pub fn follow(mut self, entity_id: Index) -> Self {
        self.follow = Some(entity_id);
        self
    }

    /// Build the `Parallax`.
    pub fn build(self) -> Parallax {
        Parallax {
            offset:     self.offset,
            speed_mult: self.speed_mult,
            follow:     self.follow,
        }
    }
}

impl Default for ParallaxBuilder {
    fn default() -> Self {
        let Parallax {
            offset,
            speed_mult,
            follow,
        } = Parallax::default();
        Self {
            offset,
            speed_mult,
            follow,
        }
    }
}
