//! Traits extending built-in Rust types

use bevy::log::prelude::*;

/// Utitities for [`Result`]
pub trait ExtendedStdResult<T> {
    fn map_err_to_string(self) -> Result<T, String>;

    /// Logs error
    fn ok_or_log_err(self) -> Option<T>;
}

impl<T, E: ToString> ExtendedStdResult<T> for Result<T, E> {
    fn map_err_to_string(self) -> Result<T, String> {
        self.map_err(|error| error.to_string())
    }

    fn ok_or_log_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                error!("{}", error.to_string());
                None
            }
        }
    }
}
