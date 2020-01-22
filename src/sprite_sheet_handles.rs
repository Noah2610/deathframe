use std::collections::HashMap;
use std::path::Path;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::{World, WorldExt};
use amethyst::renderer::sprite::{SpriteSheet, SpriteSheetHandle};
use amethyst::renderer::{ImageFormat, SpriteSheetFormat, Texture};

/// This is a resource wrapper for amethyst's `SpriteSheet`s.
/// It can load and get `SpriteSheetHandle`s;
/// _load_ them by passing a spritesheet's image file path to an appropriate method and
/// _get_ them by passing their spritesheet's image file name (without extension) to an appropriate
/// method.
#[derive(Default)]
pub struct SpriteSheetHandles {
    spritesheet_handles: HashMap<String, SpriteSheetHandle>,
}

impl SpriteSheetHandles {
    /// Insert a new `SpriteSheetHandle` with an identifier name into this resource.
    /// You will not usually use this method, instead use a method such as `load`,
    /// which handles this for you.
    pub fn insert<T>(&mut self, name: T, handle: SpriteSheetHandle)
    where
        T: ToString,
    {
        self.spritesheet_handles.insert(name.to_string(), handle);
    }

    /// Get the `SpriteSheetHandle` with the given identifier name.
    /// Returns `None` if there is no `SpriteSheetHandle` with this name,
    /// and returns `Some` with the `SpriteSheetHandle` if there is.
    pub fn get<T>(&self, name: T) -> Option<SpriteSheetHandle>
    where
        T: ToString,
    {
        let name = name.to_string();
        self.spritesheet_handles.get(&name).map(Clone::clone)
    }

    /// Load a new `SpriteSheet` and `SpriteSheetHandle` into this resource
    /// by passing the path to the spritesheet image file to this method (and the world).
    pub fn load<P>(&mut self, path: P, world: &World)
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        if !path.is_file() {
            panic!(format!(
                "Given image file path does not exist: '{:?}'",
                path
            ));
        }

        let path_ron_string = format!(
            "{}/{}.ron",
            path.parent().map(|p| p.to_str().unwrap()).unwrap_or(""),
            path.file_stem().unwrap().to_str().unwrap()
        );
        let path_ron = Path::new(path_ron_string.as_str());
        if !path_ron.is_file() {
            panic!(format!(
                "Given image file path does not have a .ron configuration \
                 file: '{:?}'",
                path_ron
            ));
        }

        let handle = {
            let loader = world.read_resource::<Loader>();
            let texture_handle = {
                let texture_storage =
                    world.read_resource::<AssetStorage<Texture>>();
                loader.load(
                    path.to_str().unwrap(),
                    ImageFormat::default(),
                    (),
                    &texture_storage,
                )
            };
            let spritesheet_store =
                world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                path_ron.to_str().unwrap(),
                SpriteSheetFormat(texture_handle),
                (),
                &spritesheet_store,
            )
        };

        let key = path.to_str().expect("Should convert path to str");
        self.insert(key, handle);
    }

    /// Get a `SpriteSheetHandle` with the given path to the spritesheet's image file.
    /// If it does not already exist, load it first, then return the newly loaded handle.
    pub fn get_or_load<P>(
        &mut self,
        path: P,
        world: &World,
    ) -> SpriteSheetHandle
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_str().expect("Should convert path to str");
        if let Some(handle) = self.get(path) {
            handle
        } else {
            self.load(path, world);
            self.get(path)
                .expect("SpriteSheet should be loaded at this point")
        }
    }

    /// Returns `true` if all `SpriteSheetHandle`s' textures have finished loading.
    pub fn has_finished_loading_all(&self, world: &World) -> bool {
        let asset = world.read_resource::<AssetStorage<SpriteSheet>>();
        self.spritesheet_handles
            .values()
            .all(|handle| asset.get(handle).is_some())
    }
}
