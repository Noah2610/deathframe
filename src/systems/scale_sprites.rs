use super::system_prelude::*;

/// This system gets all entities with `Transform`, `Size`, `SpriteRender`, and  `ScaleOnce`,
/// and scales their sprite to their entity's size _once_; after scaling, the `ScaleOnce`
/// component is removed from the entity.
pub struct ScaleSpritesSystem;

impl<'s> System<'s> for ScaleSpritesSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, AssetStorage<SpriteSheet>>,
        Read<'s, AssetStorage<Texture>>,
        ReadStorage<'s, Size>,
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, TextureHandle>,
        WriteStorage<'s, ScaleOnce>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (
            entities,
            spritesheet_asset,
            texture_asset,
            sizes,
            sprites,
            textures,
            mut scales,
            mut transforms,
        ): Self::SystemData,
    ) {
        let mut to_remove = Vec::new();
        for (
            entity,
            size,
            _scale_component,
            transform,
            sprite_render_opt,
            texture_handle_opt,
        ) in (
            &*entities,
            &sizes,
            &scales,
            &mut transforms,
            sprites.maybe(),
            textures.maybe(),
        )
            .join()
        {
            let scale_opt = if let Some(sprite_render) = sprite_render_opt {
                let spritesheet_handle = &sprite_render.sprite_sheet;
                let sprite_id = sprite_render.sprite_number;
                if let Some(spritesheet) =
                    spritesheet_asset.get(&spritesheet_handle)
                {
                    let sprite =
                        spritesheet.sprites.get(sprite_id).expect(&format!(
                            "Couldn't get sprite #{} from spritesheet #{}",
                            sprite_id,
                            spritesheet_handle.id()
                        ));
                    let sprite_w = sprite.width;
                    let sprite_h = sprite.height;
                    Some([size.w / sprite_w, size.h / sprite_h])
                } else {
                    None
                }
            } else if let Some(texture_handle) = texture_handle_opt {
                if let Some(texture) = texture_asset.get(&texture_handle) {
                    let texture_size = texture.size();
                    Some([
                        size.w / texture_size.0 as f32,
                        size.h / texture_size.1 as f32,
                    ])
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(scale) = scale_opt {
                transform.set_scale(scale[0], scale[1], 0.0);
                to_remove.push(entity);
            }
        }
        // Remove scale component from scaled entities
        for entity in to_remove {
            scales.remove(entity);
        }
    }
}
