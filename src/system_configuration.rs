use serde::Deserialize;

use crate::{bridge, error::Result, ffi, SystemConfigurationError};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
/// Wraps `SCCopyLastError` details.
pub struct SystemConfigurationLastError {
    /// Wraps the `SCCopyLastError` domain string.
    pub domain: String,
    /// Wraps the `SCCopyLastError` code.
    pub code: i64,
    /// Wraps the `SCCopyLastError` description.
    pub description: String,
    /// Wraps the `SCCopyLastError` failure reason.
    pub failure_reason: Option<String>,
    /// Wraps the `SCCopyLastError` recovery suggestion.
    pub recovery_suggestion: Option<String>,
}

#[derive(Clone, Copy, Debug, Default)]
/// Wraps top-level SystemConfiguration error helpers.
pub struct SystemConfiguration;

impl SystemConfiguration {
    /// Wraps `SCCopyLastErrorJSON`.
    pub fn copy_last_error() -> Result<Option<SystemConfigurationLastError>> {
        let raw = unsafe { ffi::system_configuration::sc_copy_last_error_json() };
        if raw.is_null() {
            return Ok(None);
        }
        bridge::parse_json("sc_copy_last_error_json", raw).map(Some)
    }

    /// Wraps `SCSystemConfigurationErrorDomain`.
    pub fn error_domain() -> Result<String> {
        bridge::take_optional_string(unsafe {
            ffi::system_configuration::sc_system_configuration_error_domain()
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_system_configuration_error_domain",
                "bridge returned null SystemConfiguration error domain",
            )
        })
    }
}
