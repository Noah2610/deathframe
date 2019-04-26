//! TODO: Documentation

use std::time::Instant;

use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

use super::component_prelude::*;

pub struct Animation {
    pub sprite_renders:        Vec<SpriteRender>,
    pub delays_ms:             Vec<u64>,
    pub last_sprite_switch_at: Instant,
    pub index:                 usize,
}

impl Animation {
    pub fn new() -> AnimationBuilder {
        AnimationBuilder::default()
    }

    pub fn current_delay_ms(&self) -> u64 {
        *self.delays_ms.get(self.index).expect(&format!(
            "Animation delay ms at index {} doesn't exist",
            self.index
        ))
    }

    pub fn current_sprite_render(&self) -> &SpriteRender {
        self.sprite_renders.get(self.index).expect(&format!(
            "Animation SpriteRender at index {} doesn't exist",
            self.index
        ))
    }
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            sprite_renders:        Vec::new(),
            delays_ms:             Vec::new(),
            last_sprite_switch_at: Instant::now(),
            index:                 0,
        }
    }
}

pub struct AnimationBuilder {
    sprite_renders:              Vec<SpriteRender>,
    delays_ms:                   Vec<u64>,
    default_sprite_sheet_handle: Option<SpriteSheetHandle>,
    default_delay_ms:            Option<u64>,
}

impl AnimationBuilder {
    pub fn sprite_renders(mut self, sprite_renders: Vec<SpriteRender>) -> Self {
        self.sprite_renders = sprite_renders;
        self
    }

    pub fn delays_ms(mut self, delays_ms: Vec<u64>) -> Self {
        self.delays_ms = delays_ms;
        self
    }

    pub fn default_sprite_sheet_handle(
        mut self,
        handle: SpriteSheetHandle,
    ) -> Self {
        self.default_sprite_sheet_handle = Some(handle);
        self
    }

    pub fn default_delay_ms(mut self, delay_ms: u64) -> Self {
        self.default_delay_ms = Some(delay_ms);
        self
    }

    pub fn sprite_ids(mut self, ids: Vec<usize>) -> Self {
        let default_handle = self.default_sprite_sheet_handle.clone().expect(
            "To use the `sprite_ids` method on the AnimationBuilder, you \
             first need to set a default sprite sheet handle with \
             `default_sprite_sheet_handle`",
        );
        self.sprite_renders = ids
            .iter()
            .map(|&id| SpriteRender {
                sprite_sheet:  default_handle.clone(),
                sprite_number: id,
            })
            .collect();
        self
    }

    pub fn build(mut self) -> Animation {
        let Animation {
            last_sprite_switch_at,
            index,
            ..
        } = Animation::default();
        let sprite_renders_len = self.sprite_renders.len();
        let delays_ms_len = self.delays_ms.len();
        if sprite_renders_len > delays_ms_len {
            let default_delay = self.default_delay_ms.expect(
                "AnimationBuilder has more `sprite_render`s than `delay_ms`s \
                 and has no default delay_ms; either make sure they have the \
                 same amount or add a default delay_ms with `default_delay_ms`",
            );
            for _ in 0 .. sprite_renders_len - delays_ms_len {
                self.delays_ms.push(default_delay);
            }
        } else if delays_ms_len > sprite_renders_len {
            panic!(
                "AnimationBuilder has more `delay_ms`s than `sprite_render`s; \
                 make sure they have the same amount"
            )
        }
        Animation {
            sprite_renders:        self.sprite_renders,
            delays_ms:             self.delays_ms,
            last_sprite_switch_at: last_sprite_switch_at,
            index:                 index,
        }
    }
}

impl Default for AnimationBuilder {
    fn default() -> Self {
        let Animation {
            sprite_renders,
            delays_ms,
            ..
        } = Animation::default();
        Self {
            sprite_renders,
            delays_ms,
            default_sprite_sheet_handle: None,
            default_delay_ms: None,
        }
    }
}
