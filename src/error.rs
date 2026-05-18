use std::{error::Error, fmt};

use crate::{bridge, ffi};

/// Result alias used by SystemConfiguration wrapper APIs.
pub type Result<T> = std::result::Result<T, SystemConfigurationError>;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Wraps SystemConfiguration framework failures.
pub struct SystemConfigurationError {
    /// Wraps the failing SystemConfiguration entry point.
    pub function: &'static str,
    /// Wraps the SystemConfiguration error code.
    pub code: i32,
    /// Wraps the SystemConfiguration error message.
    pub message: String,
}

impl SystemConfigurationError {
    pub(crate) fn new(function: &'static str, code: i32, message: impl Into<String>) -> Self {
        Self {
            function,
            code,
            message: message.into(),
        }
    }

    pub(crate) fn last(function: &'static str) -> Self {
        let code = unsafe { ffi::core::sc_last_error_code() };
        let message = bridge::take_optional_string(unsafe { ffi::core::sc_last_error_message() })
            .unwrap_or_else(|| format!("SystemConfiguration returned error code {code}"));
        Self::new(function, code, message)
    }

    pub(crate) fn null(function: &'static str, message: impl Into<String>) -> Self {
        Self::new(function, 0, message)
    }
}

impl fmt::Display for SystemConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} failed (code {}): {}",
            self.function, self.code, self.message
        )
    }
}

impl Error for SystemConfigurationError {}
