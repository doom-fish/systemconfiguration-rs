use serde::Deserialize;

use crate::{bridge, error::Result, ffi, PropertyList, SystemConfigurationError};

#[derive(Clone, Debug)]
/// Wraps `SCNetworkInterfaceRef`.
pub struct NetworkInterface {
    raw: bridge::OwnedHandle,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
/// Wraps `SCNetworkInterfaceCopyMTU` results.
pub struct NetworkInterfaceMtuInfo {
    /// Wraps the current MTU returned by `SCNetworkInterfaceCopyMTU`.
    pub current: i32,
    /// Wraps the minimum MTU returned by `SCNetworkInterfaceCopyMTU`.
    pub minimum: i32,
    /// Wraps the maximum MTU returned by `SCNetworkInterfaceCopyMTU`.
    pub maximum: i32,
}

#[derive(Clone, Debug)]
/// Wraps `SCNetworkInterfaceCopyMediaOptions` results.
pub struct NetworkInterfaceMediaOptions {
    /// Wraps the current media options from `SCNetworkInterfaceCopyMediaOptions`.
    pub current: Option<PropertyList>,
    /// Wraps the active media options from `SCNetworkInterfaceCopyMediaOptions`.
    pub active: Option<PropertyList>,
    /// Wraps the available media options from `SCNetworkInterfaceCopyMediaOptions`.
    pub available: Option<PropertyList>,
}

impl NetworkInterface {
    /// Wraps `SCNetworkInterfaceGetTypeID`.
    pub fn type_id() -> u64 {
        unsafe { ffi::network_interface::sc_network_interface_get_type_id() }
    }

    /// Wraps `SCNetworkInterfaceCopyAll`.
    pub fn copy_all() -> Result<Vec<Self>> {
        let raw = unsafe { ffi::network_interface::sc_network_interface_copy_all() };
        if raw.is_null() {
            return Err(SystemConfigurationError::last(
                "sc_network_interface_copy_all",
            ));
        }
        Ok(bridge::take_handle_array(raw, Self::from_owned_handle))
    }

    /// Wraps `SCNetworkInterfaceCopyIPv4`.
    pub fn ipv4() -> Result<Self> {
        let raw = unsafe { ffi::network_interface::sc_network_interface_copy_ipv4() };
        let raw = bridge::owned_handle_or_last("sc_network_interface_copy_ipv4", raw)?;
        Ok(Self::from_owned_handle(raw))
    }

    /// Wraps `SCNetworkInterfaceCopySupportedInterfaceTypes`.
    pub fn supported_interface_types(&self) -> Vec<String> {
        bridge::take_string_array(unsafe {
            ffi::network_interface::sc_network_interface_copy_supported_interface_types(
                self.raw.as_ptr(),
            )
        })
    }

    /// Wraps `SCNetworkInterfaceCopySupportedProtocolTypes`.
    pub fn supported_protocol_types(&self) -> Vec<String> {
        bridge::take_string_array(unsafe {
            ffi::network_interface::sc_network_interface_copy_supported_protocol_types(
                self.raw.as_ptr(),
            )
        })
    }

    /// Wraps `SCNetworkInterfaceCreateWithInterface`.
    pub fn create_layered_interface(&self, interface_type: &str) -> Result<Option<Self>> {
        let interface_type =
            bridge::cstring(interface_type, "sc_network_interface_create_with_interface")?;
        let raw = unsafe {
            ffi::network_interface::sc_network_interface_create_with_interface(
                self.raw.as_ptr(),
                interface_type.as_ptr(),
            )
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(Self::from_owned_handle))
    }

    /// Wraps `SCNetworkInterfaceCopyBSDName`.
    pub fn bsd_name(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_network_interface_copy_bsd_name(self.raw.as_ptr())
        }))
    }

    /// Wraps `SCNetworkInterfaceCopyConfiguration`.
    pub fn configuration(&self) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_interface::sc_network_interface_copy_configuration(self.raw.as_ptr()),
            )
        }
        .map(PropertyList::from_owned_handle)
    }

    /// Wraps `SCNetworkInterfaceCopyExtendedConfiguration`.
    pub fn extended_configuration(&self, extended_type: &str) -> Result<Option<PropertyList>> {
        let extended_type = bridge::cstring(
            extended_type,
            "sc_network_interface_copy_extended_configuration",
        )?;
        let raw = unsafe {
            ffi::network_interface::sc_network_interface_copy_extended_configuration(
                self.raw.as_ptr(),
                extended_type.as_ptr(),
            )
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    /// Wraps `SCNetworkInterfaceCopyHardwareAddress`.
    pub fn hardware_address_string(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_network_interface_copy_hardware_address(self.raw.as_ptr())
        }))
    }

    /// Wraps `SCNetworkInterfaceCopyUnderlyingInterface`.
    pub fn underlying_interface(&self) -> Option<Self> {
        unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_interface::sc_network_interface_copy_underlying_interface(
                    self.raw.as_ptr(),
                ),
            )
        }
        .map(Self::from_owned_handle)
    }

    /// Wraps `SCNetworkInterfaceCopyInterfaceType`.
    pub fn interface_type(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_network_interface_copy_interface_type(self.raw.as_ptr())
        }))
    }

    /// Wraps `SCNetworkInterfaceCopyLocalizedDisplayName`.
    pub fn localized_display_name(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_network_interface_copy_localized_display_name(
                self.raw.as_ptr(),
            )
        }))
    }

    /// Wraps `SCNetworkInterfaceCopyMediaOptionsCurrent`.
    pub fn media_options(&self, filter: bool) -> NetworkInterfaceMediaOptions {
        let current = unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_interface::sc_network_interface_copy_media_options_current(
                    self.raw.as_ptr(),
                    u8::from(filter),
                ),
            )
        }
        .map(PropertyList::from_owned_handle);
        let active = unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_interface::sc_network_interface_copy_media_options_active(
                    self.raw.as_ptr(),
                    u8::from(filter),
                ),
            )
        }
        .map(PropertyList::from_owned_handle);
        let available = unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_interface::sc_network_interface_copy_media_options_available(
                    self.raw.as_ptr(),
                    u8::from(filter),
                ),
            )
        }
        .map(PropertyList::from_owned_handle);
        NetworkInterfaceMediaOptions {
            current,
            active,
            available,
        }
    }

    /// Wraps `SCNetworkInterfaceCopyMediaSubtypes`.
    pub fn media_subtypes(available: &PropertyList) -> Vec<String> {
        bridge::take_string_array(unsafe {
            ffi::network_interface::sc_network_interface_copy_media_subtypes(available.as_ptr())
        })
    }

    /// Wraps `SCNetworkInterfaceCopyMediaSubtypeOptions`.
    pub fn media_subtype_options(
        available: &PropertyList,
        subtype: &str,
    ) -> Result<Vec<Vec<String>>> {
        let subtype = bridge::cstring(subtype, "sc_network_interface_copy_media_subtype_options")?;
        bridge::parse_json("sc_network_interface_copy_media_subtype_options", unsafe {
            ffi::network_interface::sc_network_interface_copy_media_subtype_options(
                available.as_ptr(),
                subtype.as_ptr(),
            )
        })
    }

    /// Wraps `SCNetworkInterfaceCopyMTUInfo`.
    pub fn mtu_info(&self) -> Result<Option<NetworkInterfaceMtuInfo>> {
        let raw = unsafe {
            ffi::network_interface::sc_network_interface_copy_mtu_info(self.raw.as_ptr())
        };
        if raw.is_null() {
            return Ok(None);
        }
        bridge::parse_json("sc_network_interface_copy_mtu_info", raw).map(Some)
    }

    /// Wraps `SCNetworkInterfaceSetConfiguration`.
    pub fn set_configuration(&self, config: Option<&PropertyList>) -> Result<()> {
        let ok = unsafe {
            ffi::network_interface::sc_network_interface_set_configuration(
                self.raw.as_ptr(),
                config.map_or(std::ptr::null_mut(), PropertyList::as_ptr),
            )
        };
        bridge::bool_result("sc_network_interface_set_configuration", ok)
    }

    /// Wraps `SCNetworkInterfaceSetExtendedConfiguration`.
    pub fn set_extended_configuration(
        &self,
        extended_type: &str,
        config: Option<&PropertyList>,
    ) -> Result<()> {
        let extended_type = bridge::cstring(
            extended_type,
            "sc_network_interface_set_extended_configuration",
        )?;
        let ok = unsafe {
            ffi::network_interface::sc_network_interface_set_extended_configuration(
                self.raw.as_ptr(),
                extended_type.as_ptr(),
                config.map_or(std::ptr::null_mut(), PropertyList::as_ptr),
            )
        };
        bridge::bool_result("sc_network_interface_set_extended_configuration", ok)
    }

    /// Wraps `SCNetworkInterfaceSetMediaOptions`.
    pub fn set_media_options<S: AsRef<str>>(
        &self,
        subtype: Option<&str>,
        options: &[S],
    ) -> Result<()> {
        let subtype = bridge::optional_cstring(subtype, "sc_network_interface_set_media_options")?;
        let options = bridge::CStringArray::new(options, "sc_network_interface_set_media_options")?;
        let ok = unsafe {
            ffi::network_interface::sc_network_interface_set_media_options(
                self.raw.as_ptr(),
                subtype
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                options.as_ptr(),
                options.count(),
            )
        };
        bridge::bool_result("sc_network_interface_set_media_options", ok)
    }

    /// Wraps `SCNetworkInterfaceSetMTU`.
    pub fn set_mtu(&self, mtu: i32) -> Result<()> {
        let ok =
            unsafe { ffi::network_interface::sc_network_interface_set_mtu(self.raw.as_ptr(), mtu) };
        bridge::bool_result("sc_network_interface_set_mtu", ok)
    }

    /// Wraps `SCNetworkInterfaceForceConfigurationRefresh`.
    pub fn force_configuration_refresh(&self) -> Result<()> {
        let ok = unsafe {
            ffi::network_interface::sc_network_interface_force_configuration_refresh(
                self.raw.as_ptr(),
            )
        };
        bridge::bool_result("sc_network_interface_force_configuration_refresh", ok)
    }

    pub(crate) fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self { raw }
    }

    pub(crate) fn as_ptr(&self) -> bridge::RawHandle {
        self.raw.as_ptr()
    }
}
