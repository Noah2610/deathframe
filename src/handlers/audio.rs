use std::collections::HashMap;
use std::path::Path;

use amethyst::assets::{AssetStorage, Format, Loader};
use amethyst::audio::output::Output;
use amethyst::audio::{
    AudioSink,
    FlacFormat,
    Mp3Format,
    OggFormat,
    Source,
    SourceHandle,
    WavFormat,
};
use amethyst::ecs::World;
use regex::RegexBuilder;

const DEFAULT_VOLUME: f32 = 1.0;

/// Custom AudioFormat enum, because amethyst removed its AudioFormat enum in v0.11.0
enum AudioFormat {
    Flac(FlacFormat),
    Mp3(Mp3Format),
    Ogg(OggFormat),
    Wav(WavFormat),
}

// impl AudioFormat {
//     fn format<F, D>(&self) -> F
//     where
//         F: Format<D>,
//     {
//         match self {
//             AudioFormat::Flac(a) => a,
//             AudioFormat::Mp3(a) => a,
//             AudioFormat::Ogg(a) => a,
//             AudioFormat::Wav(a) => a,
//         }
//     }
// }

/// This is a resource wrapper for amethyst's audio `Source`s.
/// It can load and get `SourceHandle`s;
/// _load_ them by passing a audio file path to an appropriate method and
/// _get_ them by passing the audio file name (without extension) to an appropriate method.
#[derive(Default)]
pub struct AudioHandles {
    source_handles: HashMap<String, SourceHandle>,
}

impl AudioHandles {
    /// Insert a new `SourceHandle` with an identifier name into this resource.
    /// You will not usually use this method, instead use a method such as `load`,
    /// which handles this for you.
    pub fn insert<T>(&mut self, name: T, handle: SourceHandle)
    where
        T: ToString,
    {
        self.source_handles.insert(name.to_string(), handle);
    }

    /// Get the `SourceHandle` with the given identifier name.
    /// Returns `None` if there is no `SourceHandle` with this name,
    /// and returns `Some` with the `SourceHandle` if there is.
    pub fn get<T>(&self, name: T) -> Option<SourceHandle>
    where
        T: ToString,
    {
        let name = name.to_string();
        let err_msg =
            format!("Given SourceHandle name cannot be empty: {}", name);
        // Get the basename of the file (without the extension), in case a path is passed
        let name = name
            .split("/")
            .last()
            .expect(&err_msg)
            .split(".")
            .next()
            .expect(&err_msg);
        self.source_handles.get(name).map(Clone::clone)
    }

    /// Load a new `Source` and `SourceHandle` into this resource
    /// by passing the path to the audio file to this method (and the world).
    pub fn load<P>(&mut self, path: P, world: &World)
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        if !path.is_file() {
            panic!(format!(
                "Given audio file path does not exist: '{:?}'",
                path
            ));
        }
        let err_msg_match = format!(
            "Given audio file path must match the pattern \
             `/.+\\.(ogg|wav|mp3|flac)\\z/i`. Given: '{:?}'",
            path
        );
        let filepath_regex = RegexBuilder::new(r".+\.(ogg|wav|mp3|flac)\z")
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
                    "Given path must lead to an audio file. Given: '{:?}'",
                    path
                ))
                .to_str()
                .unwrap()
                .replace(&extension_with_dot, "");

            let audio_format = match extension.to_lowercase().as_str() {
                "flac" => AudioFormat::Flac(FlacFormat),
                "mp3" => AudioFormat::Mp3(Mp3Format),
                "ogg" => AudioFormat::Ogg(OggFormat),
                "wav" => AudioFormat::Wav(WavFormat),
                ext => panic!(format!(
                    "Given format is not supported for audio: '{:?}'",
                    ext
                )),
            };

            let handle = {
                let loader = world.read_resource::<Loader>();
                // TODO: Write better code! There must be a reason why amethyst removed their AudioFormat enum!
                match audio_format {
                    AudioFormat::Flac(a) => loader.load(
                        path.to_str().unwrap(),
                        a,
                        (),
                        &world.read_resource(),
                    ),
                    AudioFormat::Mp3(a) => loader.load(
                        path.to_str().unwrap(),
                        a,
                        (),
                        &world.read_resource(),
                    ),
                    AudioFormat::Ogg(a) => loader.load(
                        path.to_str().unwrap(),
                        a,
                        (),
                        &world.read_resource(),
                    ),
                    AudioFormat::Wav(a) => loader.load(
                        path.to_str().unwrap(),
                        a,
                        (),
                        &world.read_resource(),
                    ),
                }
            };

            self.insert(name, handle);
        } else {
            panic!(err_msg_match)
        }
    }

    /// Get a `SourceHandle` with the given path to the audio file.
    /// If it does not already exist, load it first, then return the newly loaded handle.
    pub fn get_or_load<P>(&mut self, path: P, world: &World) -> SourceHandle
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

    /// Returns `true` if all `SourceHandle`s' sources have finished loading.
    pub fn has_finished_loading_all(&self, world: &World) -> bool {
        let asset = world.read_resource::<AssetStorage<Source>>();
        self.source_handles
            .values()
            .all(|handle| asset.get(handle).is_some())
    }

    /// Play the audio source with the given identifier name _once_.
    /// The world also needs to be passed.
    /// Optionally, pass a volume f32, wrapped in `Some`.
    pub fn play<T>(&self, name: T, world: &World, volume: Option<f32>)
    where
        T: ToString,
    {
        let asset = world.read_resource::<AssetStorage<Source>>();
        let output = world.read_resource::<Output>();
        self.play_with(name, &asset, &output, volume);
    }

    /// Play the audio source with the given identifier name _once_.
    /// This method differs with the `play` method by not needing to pass world,
    /// but instead you need to pass all needed data manually.
    /// This method is meant to be used from systems.
    /// Optionally, pass a volume f32, wrapped in `Some`.
    pub fn play_with<T>(
        &self,
        name: T,
        asset: &AssetStorage<Source>,
        output: &Output,
        volume: Option<f32>,
    ) where
        T: ToString,
    {
        let handle = self.get(name.to_string()).expect(&format!(
            "Audio SourceHandle with the given name doesn't exist: {}",
            name.to_string()
        ));
        if let Some(sound) = asset.get(&handle) {
            output.play_once(sound, volume.unwrap_or(DEFAULT_VOLUME));
        }
    }
}
