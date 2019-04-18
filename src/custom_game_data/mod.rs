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
    /// Helper function, which returns an amethyst error with the given message.
    pub fn amethyst_error<T>(msg: T) -> amethyst::Error
    where
        T: ToString,
    {
        use amethyst::core::bundle::{Error as BundleError, ErrorKind};
        amethyst::Error::Core(BundleError::from_kind(ErrorKind::Msg(
            msg.to_string(),
        )))
    }

    /// Helper function which returns an amethyst error with the message,
    /// that the dispatcher with the given name could not be found.
    pub fn dispatcher_not_found<T>(name: T) -> amethyst::Error
    where
        T: ToString,
    {
        amethyst_error(format!("Dispatcher not found: {}", name.to_string()))
    }
}
