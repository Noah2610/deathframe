use super::component_prelude::*;
use core::components::prelude::Size;

/// A `Hitbox` has one or more `Rect` rects,
/// which are collision boxes, relative to this entity's `Transform`.
/// So the `Rect` rects assume the entity's position is at `0, 0`.
#[derive(Debug, Component, Default, Deserialize, Clone)]
#[storage(DenseVecStorage)]
#[serde(from = "Vec<Rect>")]
pub struct Hitbox {
    pub(crate) rects: Vec<Rect>,
}

impl Hitbox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.add_rect(rect);
        self
    }

    pub fn with_rects(mut self, rects: Vec<Rect>) -> Self {
        self.rects = rects;
        self
    }

    pub fn add_rect(&mut self, rect: Rect) {
        self.rects.push(rect);
    }
}

impl From<Vec<Rect>> for Hitbox {
    fn from(rects: Vec<Rect>) -> Self {
        Self { rects }
    }
}

impl From<Rect> for Hitbox {
    fn from(rect: Rect) -> Self {
        Self { rects: vec![rect] }
    }
}

impl From<&Size> for Hitbox {
    fn from(size: &Size) -> Self {
        Self {
            rects: vec![Rect::from(size)],
        }
    }
}

impl Into<Vec<Rect>> for Hitbox {
    fn into(self) -> Vec<Rect> {
        self.rects
    }
}

impl<'a> Into<&'a Vec<Rect>> for &'a Hitbox {
    fn into(self) -> &'a Vec<Rect> {
        &self.rects
    }
}
