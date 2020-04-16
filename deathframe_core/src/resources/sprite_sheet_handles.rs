use amethyst::assets::{AssetStorage, Loader};
use amethyst::ecs::{World, WorldExt};
use amethyst::renderer::sprite::{SpriteSheet, SpriteSheetHandle};
use amethyst::renderer::{ImageFormat, SpriteSheetFormat, Texture};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::path::Path;

/// This is a resource wrapper for amethyst's `SpriteSheet`s.
/// It can load and get `SpriteSheetHandle`s;
#[derive(Default)]
pub struct SpriteSheetHandles<K>
where
    K: PartialEq + Eq + Hash + Debug,
{
    handles: HashMap<K, SpriteSheetHandle>,
}

impl<K> SpriteSheetHandles<K>
where
    K: PartialEq + Eq + Hash + Debug,
{
    /// Insert a new `SpriteSheetHandle` with an identifier name into this resource.
    /// You will not usually use this method, instead use a method such as `load`,
    /// which handles this for you.
    pub fn insert(&mut self, key: K, handle: SpriteSheetHandle) {
        self.handles.insert(key, handle);
    }

    /// Get the `SpriteSheetHandle` with the given identifier name.
    /// Returns `None` if there is no `SpriteSheetHandle` with this name,
    /// and returns `Some` with the `SpriteSheetHandle` if there is.
    pub fn get(&self, key: &K) -> Option<SpriteSheetHandle> {
        self.handles.get(key).cloned()
    }

    /// Load a new `SpriteSheet` and `SpriteSheetHandle` into this resource
    /// by passing the path to the spritesheet image file to this method (and the world).
    pub fn load<P>(&mut self, name: P, world: &World)
    where
        P: AsRef<Path> + Into<K> + Clone,
    {
        let key = name.clone().into();
        let path = name.as_ref();

        if !path.is_file() {
            panic!(format!(
                "Given image file path does not exist: '{:?}'",
                path
            ));
        }

        let path_ron =
            path.parent().expect("Image path should have parent").join({
                let mut ron = path
                    .file_stem()
                    .expect("Image path should have file stem")
                    .to_owned();
                ron.push(".ron");
                ron
            });

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

        self.insert(key, handle);
    }

    /// Get a `SpriteSheetHandle` with the given path to the spritesheet's image file.
    /// If it does not already exist, load it first, then return the newly loaded handle.
    pub fn get_or_load<P>(
        &mut self,
        name: P,
        world: &World,
    ) -> SpriteSheetHandle
    where
        P: AsRef<Path> + Into<K> + Clone,
    {
        let key: K = name.clone().into();

        if let Some(handle) = self.get(&key) {
            handle
        } else {
            self.load(name, world);
            self.get(&key)
                .expect("SpriteSheet should be loaded at this point")
        }
    }

    /// Returns `true` if all `SpriteSheetHandle`s' textures have finished loading.
    pub fn has_finished_loading_all(&self, world: &World) -> bool {
        let asset = world.read_resource::<AssetStorage<SpriteSheet>>();
        self.handles
            .values()
            .all(|handle| asset.get(handle).is_some())
    }
}
