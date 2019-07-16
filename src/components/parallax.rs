use amethyst::ecs::world::Index;

use super::component_prelude::*;
use crate::geo::{Anchor, Vector};

/// A parallax background. This background has an offset and it can follow another entity,
/// similar to the camera. It is meant to be used with more parallax backgrounds.
/// The speed multiplier determines at what speed the image moves, relative to the following
/// entity. (Does not use velocity.)
#[derive(Serialize, Deserialize)]
pub struct Parallax {
    /// Positional offset.
    pub offset: Vector,
    /// Speed multipliers, values should be between 0.0 and 1.0.
    pub speed_mult: Vector,
    /// The ID of the entity to follow; doesn't make much sense without this.
    pub follow: Option<Index>,
    /// The anchor point of the following entity. Usually `Anchor::Middle`.
    pub follow_anchor: Anchor,
}

impl Parallax {
    /// Returns a new `ParallaxBuilder`.
    pub fn new() -> ParallaxBuilder {
        ParallaxBuilder::default()
    }

    /// Calculate the new position for this parallax given
    /// the following entity's position and size `Option`.
    pub fn calculate_pos_with_following(
        &self,
        following_pos: Vector,
        following_size_opt: Option<Vector>,
    ) -> Vector {
        let following_middle = if let Some(following_size) = following_size_opt
        {
            self.follow_anchor
                .middle_for(&following_pos, &following_size)
        } else {
            following_pos
        };

        Vector::new(
            following_middle.0 + self.offset.0
                - following_middle.0 * self.speed_mult.0,
            following_middle.1 + self.offset.1
                - following_middle.1 * self.speed_mult.1,
        )
        // Vector::new(
        //     following_middle.0 * self.speed_mult.0 + self.offset.0,
        //     following_middle.1 * self.speed_mult.1 + self.offset.1,
        // )
    }

    /// Calculate the new position for this parallax given
    /// the following entity's position and size (_not optional!_),
    /// _and_ it should repeat on the given axes (`repeat_x`, `repeat_y`).
    /// Repetition means, the calculated position _must_ be within the following's area.
    pub fn calculate_pos_with_following_with_repeat(
        &self,
        following_pos: Vector,
        following_size: Vector,
        repeat_x: bool,
        repeat_y: bool,
    ) -> Vector {
        let following_middle = self
            .follow_anchor
            .middle_for(&following_pos, &following_size);

        Vector::new(
            if repeat_x {
                following_middle.0 + self.offset.0
                    - (following_middle.0 * self.speed_mult.0
                        % following_size.0)
            } else {
                following_middle.0 + self.offset.0
                    - following_middle.0 * self.speed_mult.0
            },
            if repeat_y {
                following_middle.1 + self.offset.1
                    - (following_middle.1 * self.speed_mult.1
                        % following_size.1)
            } else {
                following_middle.1 + self.offset.1
                    - following_middle.1 * self.speed_mult.1
            },
        )
    }
}

impl Component for Parallax {
    type Storage = HashMapStorage<Self>;
}

impl Default for Parallax {
    fn default() -> Self {
        Self {
            offset:        (0.0, 0.0).into(),
            speed_mult:    (0.5, 0.5).into(),
            follow:        None,
            follow_anchor: Anchor::Middle,
        }
    }
}

/// A builder struct for `Parallax`.
pub struct ParallaxBuilder {
    offset:        Vector,
    speed_mult:    Vector,
    follow:        Option<Index>,
    follow_anchor: Anchor,
}

impl ParallaxBuilder {
    /// Set the `offset`.
    pub fn offset(mut self, offset: Vector) -> Self {
        self.offset = offset;
        self
    }

    /// Set the `x offset`.
    pub fn offset_x(mut self, x: f32) -> Self {
        self.offset.0 = x;
        self
    }

    /// Set the `y offset`.
    pub fn offset_y(mut self, y: f32) -> Self {
        self.offset.1 = y;
        self
    }

    /// Set the `speed_mult`.
    pub fn speed_mult(mut self, speed_mult: Vector) -> Self {
        self.speed_mult = speed_mult;
        self
    }

    /// Set the `x speed_mult`.
    pub fn speed_mult_x(mut self, x: f32) -> Self {
        self.speed_mult.0 = x;
        self
    }

    /// Set the `y speed_mult`.
    pub fn speed_mult_y(mut self, y: f32) -> Self {
        self.speed_mult.1 = y;
        self
    }

    /// Set the entity ID to follow.
    pub fn follow(mut self, entity_id: Index) -> Self {
        self.follow = Some(entity_id);
        self
    }

    /// Set the anchor point of the following entity.
    pub fn follow_anchor(mut self, anchor: Anchor) -> Self {
        self.follow_anchor = anchor;
        self
    }

    /// Build the `Parallax`.
    pub fn build(self) -> Parallax {
        Parallax {
            offset:        self.offset,
            speed_mult:    self.speed_mult,
            follow:        self.follow,
            follow_anchor: self.follow_anchor,
        }
    }
}

impl Default for ParallaxBuilder {
    fn default() -> Self {
        let Parallax {
            offset,
            speed_mult,
            follow,
            follow_anchor,
        } = Parallax::default();
        Self {
            offset,
            speed_mult,
            follow,
            follow_anchor,
        }
    }
}
