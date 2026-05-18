use crate::{
    bridge, error::Result, ffi, network_interface::NetworkInterface, preferences::Preferences,
    PropertyList,
};

#[derive(Clone)]
/// Wraps `SCVLANInterfaceRef`.
pub struct VlanInterface {
    raw: bridge::OwnedHandle,
}

impl std::fmt::Debug for VlanInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VlanInterface").finish_non_exhaustive()
    }
}

impl VlanInterface {
    /// Wraps `SCVLANInterfaceCopyAll`.
    pub fn copy_all(preferences: &Preferences) -> Vec<Self> {
        let raw =
            unsafe { ffi::network_interface::sc_vlan_interface_copy_all(preferences.as_ptr()) };
        bridge::take_handle_array(raw, Self::from_owned_handle)
    }

    /// Wraps `SCVLANInterfaceCopyAvailablePhysicalInterfaces`.
    pub fn copy_available_physical_interfaces() -> Vec<NetworkInterface> {
        let raw = unsafe {
            ffi::network_interface::sc_vlan_interface_copy_available_physical_interfaces()
        };
        bridge::take_handle_array(raw, NetworkInterface::from_owned_handle)
    }

    /// Wraps `SCVLANInterfaceCreate`.
    pub fn create(
        preferences: &Preferences,
        physical: &NetworkInterface,
        tag: i32,
    ) -> Result<Self> {
        let raw = unsafe {
            ffi::network_interface::sc_vlan_interface_create(
                preferences.as_ptr(),
                physical.as_ptr(),
                tag,
            )
        };
        let raw = bridge::owned_handle_or_last("sc_vlan_interface_create", raw)?;
        Ok(Self { raw })
    }

    /// Wraps `SCVLANInterfaceCopyOptions`.
    pub fn options(&self) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(ffi::network_interface::sc_vlan_interface_copy_options(
                self.raw.as_ptr(),
            ))
        }
        .map(PropertyList::from_owned_handle)
    }

    /// Wraps `SCVLANInterfaceCopyPhysicalInterface`.
    pub fn physical_interface(&self) -> Option<NetworkInterface> {
        unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_interface::sc_vlan_interface_copy_physical_interface(
                    self.raw.as_ptr(),
                ),
            )
        }
        .map(NetworkInterface::from_owned_handle)
    }

    /// Wraps `SCVLANInterfaceGetTag`.
    pub fn tag(&self) -> Result<Option<i32>> {
        let mut tag = 0_i32;
        let ok = unsafe {
            ffi::network_interface::sc_vlan_interface_get_tag(self.raw.as_ptr(), &mut tag)
        };
        if ok == 0 {
            return Ok(None);
        }
        Ok(Some(tag))
    }

    /// Wraps `SCVLANInterfaceRemove`.
    pub fn remove(&self) -> Result<()> {
        let ok = unsafe { ffi::network_interface::sc_vlan_interface_remove(self.raw.as_ptr()) };
        bridge::bool_result("sc_vlan_interface_remove", ok)
    }

    /// Wraps `SCVLANInterfaceSetLocalizedDisplayName`.
    pub fn set_localized_display_name(&self, name: &str) -> Result<()> {
        let name = bridge::cstring(name, "sc_vlan_interface_set_localized_display_name")?;
        let ok = unsafe {
            ffi::network_interface::sc_vlan_interface_set_localized_display_name(
                self.raw.as_ptr(),
                name.as_ptr(),
            )
        };
        bridge::bool_result("sc_vlan_interface_set_localized_display_name", ok)
    }

    /// Wraps `SCVLANInterfaceSetOptions`.
    pub fn set_options(&self, options: &PropertyList) -> Result<()> {
        let ok = unsafe {
            ffi::network_interface::sc_vlan_interface_set_options(
                self.raw.as_ptr(),
                options.as_ptr(),
            )
        };
        bridge::bool_result("sc_vlan_interface_set_options", ok)
    }

    /// Wraps `SCVLANInterfaceSetPhysicalInterfaceAndTag`.
    pub fn set_physical_interface_and_tag(
        &self,
        physical: &NetworkInterface,
        tag: i32,
    ) -> Result<()> {
        let ok = unsafe {
            ffi::network_interface::sc_vlan_interface_set_physical_interface_and_tag(
                self.raw.as_ptr(),
                physical.as_ptr(),
                tag,
            )
        };
        bridge::bool_result("sc_vlan_interface_set_physical_interface_and_tag", ok)
    }

    /// Wraps a helper on `SCVLANInterfaceRef`.
    pub fn as_network_interface(&self) -> NetworkInterface {
        NetworkInterface::from_owned_handle(self.raw.clone())
    }

    pub(crate) fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self { raw }
    }
}
