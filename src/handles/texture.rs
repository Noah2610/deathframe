use std::collections::HashMap;
use std::path::Path;

use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::ecs::World;
use amethyst::renderer::{ImageFormat, Texture};
use regex::RegexBuilder;

type TextureHandle = Handle<Texture>;

/// This is a resource wrapper for amethyst's `Texture`s.
/// It can load and get `TextureHandle`s;
/// _load_ them by passing a texture's image file path to an appropriate method and
/// _get_ them by passing their texture's image file name (without extension) to an appropriate
/// method.
#[derive(Default)]
pub struct TextureHandles {
    texture_handles: HashMap<String, TextureHandle>,
}

impl TextureHandles {
    /// Insert a new `TextureHandle` with an identifier name into this resource.
    /// You will not usually use this method, instead use a method such as `load`,
    /// which handles this for you.
    pub fn insert<T>(&mut self, name: T, handle: TextureHandle)
    where
        T: ToString,
    {
        self.texture_handles.insert(name.to_string(), handle);
    }

    /// Get the `TextureHandle` with the given identifier name.
    /// Returns `None` if there is no `TextureHandle` with this name,
    /// and returns `Some` with the `TextureHandle` if there is.
    pub fn get<T>(&self, name: T) -> Option<TextureHandle>
    where
        T: ToString,
    {
        let name = name.to_string();
        let err_msg =
            format!("Given TextureHandle name cannot be empty: {}", name);
        // Get the basename of the file (without the extension), in case a path is passed
        let name = name
            .split("/")
            .last()
            .expect(&err_msg)
            .split(".")
            .next()
            .expect(&err_msg);
        self.texture_handles.get(name).map(Clone::clone)
    }

    /// Load a new `Texture` and `TextureHandle` into this resource
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
        let err_msg_match = format!(
            "Given image file path must match the pattern \
             `/.+\\.(png|jpe?g)\\z/i`. Given: '{:?}'",
            path
        );
        let filepath_regex = RegexBuilder::new(r".+\.(png|jpe?g)\z")
            .case_insensitive(true)
            .build()
            .unwrap();

        if filepath_regex.captures(path.to_str().unwrap()).is_some() {
            let extension =
                path.extension().expect(&err_msg_match).to_str().unwrap();
            let extension_with_dot = format!(".{}", extension);
            let name = path
                .file_name()
                .expect(&format!(
                    "Given path must lead to an image file. Given: '{:?}'",
                    path
                ))
                .to_str()
                .unwrap()
                .replace(&extension_with_dot, "");

            let handle = {
                let loader = world.read_resource::<Loader>();
                let texture_storage =
                    world.read_resource::<AssetStorage<Texture>>();
                loader.load(
                    path.to_str().unwrap(),
                    ImageFormat::default(),
                    (),
                    &texture_storage,
                )
            };

            self.insert(name, handle);
        } else {
            panic!(err_msg_match)
        }
    }

    /// Get a `TextureHandle` with the given path to the spritesheet's image file.
    /// If it does not already exist, load it first, then return the newly loaded handle.
    pub fn get_or_load<P>(&mut self, path: P, world: &World) -> TextureHandle
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_str().unwrap();
        if let Some(handle) = self.get(path) {
            handle
        } else {
            self.load(path, world);
            self.get(path).unwrap()
        }
    }

    /// Returns `true` if all `TextureHandle`s' textures have finished loading.
    pub fn has_finished_loading_all(&self, world: &World) -> bool {
        let asset = world.read_resource::<AssetStorage<Texture>>();
        self.texture_handles
            .values()
            .all(|handle| asset.get(handle).is_some())
    }
}
