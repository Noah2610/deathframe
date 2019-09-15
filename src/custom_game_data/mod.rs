mod builder;
mod data;

pub mod prelude {
    pub use super::CustomGameData;
    pub use super::CustomGameDataBuilder;
}

pub use builder::CustomGameDataBuilder;
pub use data::CustomGameData;

/// Module containing helper function(s);
/// intended only for use within this module.
mod internal_helpers {
    /// Helper function which returns an amethyst error with the message,
    /// that the dispatcher with the given name could not be found.
    pub fn dispatcher_not_found<T>(name: T) -> amethyst::Error
    where
        T: ToString,
    {
        amethyst::Error::from_string(format!(
            "Dispatcher not found: {}",
            name.to_string()
        ))
    }
}
