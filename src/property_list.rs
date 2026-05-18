use crate::{bridge, error::Result, ffi, SystemConfigurationError};

#[derive(Clone, Debug)]
/// Wraps `CFPropertyListRef` values used by SystemConfiguration APIs.
pub struct PropertyList {
    raw: bridge::OwnedHandle,
}

impl PropertyList {
    /// Wraps `SCPropertyListFromString`.
    pub fn from_string(value: &str) -> Result<Self> {
        let value = bridge::cstring(value, "sc_property_list_from_string")?;
        let raw = unsafe { ffi::core::sc_property_list_from_string(value.as_ptr()) };
        let raw = bridge::owned_handle_or_last("sc_property_list_from_string", raw)?;
        Ok(Self { raw })
    }

    /// Wraps `SCPropertyListFromBool`.
    pub fn from_bool(value: bool) -> Self {
        let raw = unsafe { ffi::core::sc_property_list_from_bool(u8::from(value)) };
        let raw = bridge::owned_handle_or_last("sc_property_list_from_bool", raw)
            .expect("sc_property_list_from_bool returned null");
        Self { raw }
    }

    /// Wraps `SCPropertyListFromJSON`.
    pub fn from_json(value: &str) -> Result<Self> {
        let value = bridge::cstring(value, "sc_property_list_from_json")?;
        let raw = unsafe { ffi::core::sc_property_list_from_json(value.as_ptr()) };
        let raw = bridge::owned_handle_or_last("sc_property_list_from_json", raw)?;
        Ok(Self { raw })
    }

    /// Wraps `SCPropertyListCopyDescription`.
    pub fn description(&self) -> Result<String> {
        let raw = unsafe { ffi::core::sc_property_list_copy_description(self.raw.as_ptr()) };
        bridge::take_optional_string(raw).ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_property_list_copy_description",
                "bridge returned null property-list description",
            )
        })
    }

    pub(crate) fn as_ptr(&self) -> bridge::RawHandle {
        self.raw.as_ptr()
    }

    pub(crate) fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self { raw }
    }
}
