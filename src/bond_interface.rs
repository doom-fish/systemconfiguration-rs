use crate::{
    bridge::{self, HandleArray},
    error::Result,
    ffi,
    network_interface::NetworkInterface,
    preferences::Preferences,
    PropertyList, SystemConfigurationError,
};

#[derive(Clone)]
pub struct BondInterface {
    raw: bridge::OwnedHandle,
}

impl std::fmt::Debug for BondInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BondInterface").finish_non_exhaustive()
    }
}

#[derive(Clone)]
pub struct BondStatus {
    raw: bridge::OwnedHandle,
}

impl std::fmt::Debug for BondStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BondStatus").finish_non_exhaustive()
    }
}

impl BondInterface {
    pub fn copy_all(preferences: &Preferences) -> Vec<Self> {
        let raw = unsafe { ffi::network_interface::sc_bond_interface_copy_all(preferences.as_ptr()) };
        bridge::take_handle_array(raw, Self::from_owned_handle)
    }

    pub fn copy_available_member_interfaces(preferences: &Preferences) -> Vec<NetworkInterface> {
        let raw = unsafe {
            ffi::network_interface::sc_bond_interface_copy_available_member_interfaces(preferences.as_ptr())
        };
        bridge::take_handle_array(raw, NetworkInterface::from_owned_handle)
    }

    pub fn create(preferences: &Preferences) -> Result<Self> {
        let raw = unsafe { ffi::network_interface::sc_bond_interface_create(preferences.as_ptr()) };
        let raw = bridge::owned_handle_or_last("sc_bond_interface_create", raw)?;
        Ok(Self { raw })
    }

    pub fn member_interfaces(&self) -> Vec<NetworkInterface> {
        let raw = unsafe { ffi::network_interface::sc_bond_interface_copy_member_interfaces(self.raw.as_ptr()) };
        bridge::take_handle_array(raw, NetworkInterface::from_owned_handle)
    }

    pub fn options(&self) -> Option<PropertyList> {
        unsafe { bridge::OwnedHandle::from_raw(ffi::network_interface::sc_bond_interface_copy_options(self.raw.as_ptr())) }
            .map(PropertyList::from_owned_handle)
    }

    pub fn remove(&self) -> Result<()> {
        let ok = unsafe { ffi::network_interface::sc_bond_interface_remove(self.raw.as_ptr()) };
        bridge::bool_result("sc_bond_interface_remove", ok)
    }

    pub fn set_localized_display_name(&self, name: &str) -> Result<()> {
        let name = bridge::cstring(name, "sc_bond_interface_set_localized_display_name")?;
        let ok = unsafe {
            ffi::network_interface::sc_bond_interface_set_localized_display_name(self.raw.as_ptr(), name.as_ptr())
        };
        bridge::bool_result("sc_bond_interface_set_localized_display_name", ok)
    }

    pub fn set_member_interfaces(&self, members: &[NetworkInterface]) -> Result<()> {
        let members = HandleArray::new(members, NetworkInterface::as_ptr);
        let ok = unsafe {
            ffi::network_interface::sc_bond_interface_set_member_interfaces(
                self.raw.as_ptr(),
                members.as_ptr(),
                members.count(),
            )
        };
        bridge::bool_result("sc_bond_interface_set_member_interfaces", ok)
    }

    pub fn set_options(&self, options: &PropertyList) -> Result<()> {
        let ok = unsafe {
            ffi::network_interface::sc_bond_interface_set_options(self.raw.as_ptr(), options.as_ptr())
        };
        bridge::bool_result("sc_bond_interface_set_options", ok)
    }

    pub fn status(&self) -> Option<BondStatus> {
        unsafe { bridge::OwnedHandle::from_raw(ffi::network_interface::sc_bond_interface_copy_status(self.raw.as_ptr())) }
            .map(BondStatus::from_owned_handle)
    }

    pub fn as_network_interface(&self) -> NetworkInterface {
        NetworkInterface::from_owned_handle(self.raw.clone())
    }

    pub(crate) fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self { raw }
    }

}

impl BondStatus {
    pub fn type_id() -> u64 {
        unsafe { ffi::network_interface::sc_bond_status_get_type_id() }
    }

    pub fn device_aggregation_status_key() -> Result<String> {
        bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_bond_status_copy_device_aggregation_status_key()
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_bond_status_copy_device_aggregation_status_key",
                "bridge returned null bond aggregation-status key",
            )
        })
    }

    pub fn device_collecting_key() -> Result<String> {
        bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_bond_status_copy_device_collecting_key()
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_bond_status_copy_device_collecting_key",
                "bridge returned null bond collecting-status key",
            )
        })
    }

    pub fn device_distributing_key() -> Result<String> {
        bridge::take_optional_string(unsafe {
            ffi::network_interface::sc_bond_status_copy_device_distributing_key()
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_bond_status_copy_device_distributing_key",
                "bridge returned null bond distributing-status key",
            )
        })
    }

    pub fn member_interfaces(&self) -> Vec<NetworkInterface> {
        let raw = unsafe { ffi::network_interface::sc_bond_status_copy_member_interfaces(self.raw.as_ptr()) };
        bridge::take_handle_array(raw, NetworkInterface::from_owned_handle)
    }

    pub fn interface_status(&self, interface: Option<&NetworkInterface>) -> Option<PropertyList> {
        let raw = unsafe {
            ffi::network_interface::sc_bond_status_copy_interface_status(
                self.raw.as_ptr(),
                interface.map_or(std::ptr::null_mut(), NetworkInterface::as_ptr),
            )
        };
        unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle)
    }

    pub fn bond_status(&self) -> Result<PropertyList> {
        self.interface_status(None).ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_bond_status_copy_interface_status",
                "bridge returned null bond status dictionary",
            )
        })
    }

    pub(crate) fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self { raw }
    }
}
