use serde::Deserialize;

use crate::{bridge, error::Result, ffi, PropertyList, SystemConfigurationError};

#[derive(Clone, Debug)]
pub struct NetworkInterface {
    raw: bridge::OwnedHandle,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct NetworkInterfaceMtuInfo {
    pub current: i32,
    pub minimum: i32,
    pub maximum: i32,
}

impl NetworkInterface {
    pub fn copy_all() -> Result<Vec<Self>> {
        let raw = unsafe { ffi::network_interface::sc_network_interface_copy_all() };
        if raw.is_null() {
            return Err(SystemConfigurationError::last("sc_network_interface_copy_all"));
        }
        Ok(bridge::take_handle_array(raw, Self::from_owned_handle))
    }

    pub fn supported_interface_types(&self) -> Vec<String> {
        bridge::take_string_array(unsafe {
            ffi::network_interface::sc_network_interface_copy_supported_interface_types(self.raw.as_ptr())
        })
    }

    pub fn supported_protocol_types(&self) -> Vec<String> {
        bridge::take_string_array(unsafe {
            ffi::network_interface::sc_network_interface_copy_supported_protocol_types(self.raw.as_ptr())
        })
    }

    pub fn create_layered_interface(&self, interface_type: &str) -> Result<Option<Self>> {
        let interface_type = bridge::cstring(interface_type, "sc_network_interface_create_with_interface")?;
        let raw = unsafe {
            ffi::network_interface::sc_network_interface_create_with_interface(
                self.raw.as_ptr(),
                interface_type.as_ptr(),
            )
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(Self::from_owned_handle))
    }

    pub fn bsd_name(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_network_interface_copy_bsd_name(self.raw.as_ptr())
        }))
    }

    pub fn configuration(&self) -> Option<PropertyList> {
        unsafe { bridge::OwnedHandle::from_raw(ffi::network_interface::sc_network_interface_copy_configuration(self.raw.as_ptr())) }
            .map(PropertyList::from_owned_handle)
    }

    pub fn extended_configuration(&self, extended_type: &str) -> Result<Option<PropertyList>> {
        let extended_type = bridge::cstring(extended_type, "sc_network_interface_copy_extended_configuration")?;
        let raw = unsafe {
            ffi::network_interface::sc_network_interface_copy_extended_configuration(
                self.raw.as_ptr(),
                extended_type.as_ptr(),
            )
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    pub fn hardware_address_string(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_network_interface_copy_hardware_address(self.raw.as_ptr())
        }))
    }

    pub fn underlying_interface(&self) -> Option<Self> {
        unsafe {
            bridge::OwnedHandle::from_raw(ffi::network_interface::sc_network_interface_copy_underlying_interface(
                self.raw.as_ptr(),
            ))
        }
        .map(Self::from_owned_handle)
    }

    pub fn interface_type(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_network_interface_copy_interface_type(self.raw.as_ptr())
        }))
    }

    pub fn localized_display_name(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_network_interface_copy_localized_display_name(self.raw.as_ptr())
        }))
    }

    pub fn mtu_info(&self) -> Result<Option<NetworkInterfaceMtuInfo>> {
        let raw = unsafe { ffi::network_interface::sc_network_interface_copy_mtu_info(self.raw.as_ptr()) };
        if raw.is_null() {
            return Ok(None);
        }
        bridge::parse_json("sc_network_interface_copy_mtu_info", raw).map(Some)
    }

    pub fn force_configuration_refresh(&self) -> Result<()> {
        let ok = unsafe { ffi::network_interface::sc_network_interface_force_configuration_refresh(self.raw.as_ptr()) };
        bridge::bool_result("sc_network_interface_force_configuration_refresh", ok)
    }

    pub(crate) fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self { raw }
    }

    pub(crate) fn as_ptr(&self) -> bridge::RawHandle {
        self.raw.as_ptr()
    }
}
