use std::{error::Error, ffi::CStr, fmt};

use crate::ffi;

pub type Result<T> = std::result::Result<T, SystemConfigurationError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemConfigurationError {
    pub function: &'static str,
    pub code: i32,
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
        let code = unsafe { ffi::SCError() };
        let message = unsafe {
            let raw = ffi::SCErrorString(code);
            if raw.is_null() {
                format!("SystemConfiguration returned error code {code}")
            } else {
                CStr::from_ptr(raw).to_string_lossy().into_owned()
            }
        };

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
