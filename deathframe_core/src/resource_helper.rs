use amethyst::utils::app_root_dir::application_dir;
use std::path::PathBuf;

/// Helper function, that returns a `PathBuf` to the resource file
/// under the project's `resources` directory.
/// Converts forward-slashes "/" to back-slashes "\", if OS target is windows.
/// With the `debug` feature, this function will also
/// print a warning message to stderr if the file doesn't exist.
pub fn resource<P>(name: P) -> PathBuf
where
    P: Into<PathBuf>,
{
    let path = application_dir("resources")
        .expect("Should have resources directory")
        .join(
            if cfg!(target_os = "windows") {
                name.into().to_str().unwrap().replace("/", "\\").into()
            } else {
                name.into()
            },
        );
    #[cfg(feature = "debug")]
    {
        if !path.exists() {
            eprintln!(
                "[WARNING]\n    Resource path doesn't exist: {:?}",
                &path
            );
        }
    }
    path
}
