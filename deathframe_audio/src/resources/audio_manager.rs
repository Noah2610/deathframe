use amethyst::assets::{AssetStorage, Loader};
use amethyst::audio::{
    FlacFormat,
    Mp3Format,
    OggFormat,
    Source,
    SourceHandle,
    WavFormat,
};
use core::amethyst;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::path::Path;

/// Trait with common functions between the sound/music managers.
pub trait AudioManager<K>
where
    K: PartialEq + Eq + Hash,
{
    /// Has to return a reference to the `HashMap`,
    /// containing loaded `SourceHandle`s.
    fn get_source_handles(&self) -> &HashMap<K, SourceHandle>;

    /// Has to return a mutable reference to the `HashMap`,
    /// containing loaded `SourceHandle`s.
    fn mut_source_handles(&mut self) -> &mut HashMap<K, SourceHandle>;

    /// Load sound file from for key `K` from the given path.
    /// The file format is derived from the filename's extension.
    /// Returns an Error, if no matching audio format was found
    /// for the file extension.
    /// Valid extensions: ".wav", ".mp3", ".ogg", ".flac"
    fn load_sound<P>(
        &mut self,
        key: K,
        path: P,
        loader: &Loader,
        asset_storage: &AssetStorage<Source>,
    ) -> Result<(), String>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let audio_format = AudioFormat::try_from(
            path.extension()
                .ok_or(format!(
                    "No extension for file, cannot figure out audio file \
                     format: {:?}",
                    &path
                ))?
                .to_str()
                .ok_or("Couldn't convert path extension to string")?,
        )?;
        let filepath =
            path.to_str().ok_or("Couldn't convert path to string")?;
        self.mut_source_handles().insert(key, match audio_format {
            AudioFormat::Flac(format) => {
                loader.load(filepath, format, (), asset_storage)
            }
            AudioFormat::Mp3(format) => {
                loader.load(filepath, format, (), asset_storage)
            }
            AudioFormat::Ogg(format) => {
                loader.load(filepath, format, (), asset_storage)
            }
            AudioFormat::Wav(format) => {
                loader.load(filepath, format, (), asset_storage)
            }
        });
        Ok(())
    }

    /// Get the `SourceHandle` for the given key `K`.
    fn get_handle<'a, 'b>(&'a self, key: &'b K) -> Option<&'a SourceHandle>
    where
        'b: 'a,
    {
        self.get_source_handles().get(key)
    }
}

enum AudioFormat {
    Flac(FlacFormat),
    Mp3(Mp3Format),
    Ogg(OggFormat),
    Wav(WavFormat),
}

impl TryFrom<&str> for AudioFormat {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().trim() {
            "flac" => Ok(AudioFormat::Flac(FlacFormat)),
            "mp3" => Ok(AudioFormat::Mp3(Mp3Format)),
            "ogg" => Ok(AudioFormat::Ogg(OggFormat)),
            "wav" => Ok(AudioFormat::Wav(WavFormat)),
            _ => Err(format!("Invalid extension for any audio format: {}", s)),
        }
    }
}
