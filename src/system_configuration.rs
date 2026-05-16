use serde::Deserialize;

use crate::{bridge, error::Result, ffi, SystemConfigurationError};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SystemConfigurationLastError {
    pub domain: String,
    pub code: i64,
    pub description: String,
    pub failure_reason: Option<String>,
    pub recovery_suggestion: Option<String>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemConfiguration;

impl SystemConfiguration {
    pub fn copy_last_error() -> Result<Option<SystemConfigurationLastError>> {
        let raw = unsafe { ffi::system_configuration::sc_copy_last_error_json() };
        if raw.is_null() {
            return Ok(None);
        }
        bridge::parse_json("sc_copy_last_error_json", raw).map(Some)
    }

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
