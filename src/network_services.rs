use crate::{
    bridge, error::Result, ffi, network_interface::NetworkInterface,
    network_protocol::NetworkProtocol, preferences::Preferences,
};

#[derive(Clone, Debug)]
pub struct NetworkService {
    raw: bridge::OwnedHandle,
}

impl NetworkService {
    pub fn type_id() -> u64 {
        unsafe { ffi::network_services::sc_network_service_get_type_id() }
    }

    pub fn copy_all(preferences: &Preferences) -> Vec<Self> {
        let raw =
            unsafe { ffi::network_services::sc_network_service_copy_all(preferences.as_ptr()) };
        bridge::take_handle_array(raw, Self::from_owned_handle)
    }

    pub fn create(preferences: &Preferences, interface: &NetworkInterface) -> Result<Self> {
        let raw = unsafe {
            ffi::network_services::sc_network_service_create(
                preferences.as_ptr(),
                interface.as_ptr(),
            )
        };
        let raw = bridge::owned_handle_or_last("sc_network_service_create", raw)?;
        Ok(Self { raw })
    }

    pub fn copy(preferences: &Preferences, service_id: &str) -> Result<Option<Self>> {
        let service_id = bridge::cstring(service_id, "sc_network_service_copy")?;
        let raw = unsafe {
            ffi::network_services::sc_network_service_copy(
                preferences.as_ptr(),
                service_id.as_ptr(),
            )
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(Self::from_owned_handle))
    }

    pub fn copy_protocols(&self) -> Vec<NetworkProtocol> {
        let raw =
            unsafe { ffi::network_services::sc_network_service_copy_protocols(self.raw.as_ptr()) };
        bridge::take_handle_array(raw, NetworkProtocol::from_owned_handle)
    }

    pub fn copy_protocol(&self, protocol_type: &str) -> Result<Option<NetworkProtocol>> {
        let protocol_type = bridge::cstring(protocol_type, "sc_network_service_copy_protocol")?;
        let raw = unsafe {
            ffi::network_services::sc_network_service_copy_protocol(
                self.raw.as_ptr(),
                protocol_type.as_ptr(),
            )
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(NetworkProtocol::from_owned_handle))
    }

    pub fn add_protocol_type(&self, protocol_type: &str) -> Result<()> {
        let protocol_type = bridge::cstring(protocol_type, "sc_network_service_add_protocol_type")?;
        let ok = unsafe {
            ffi::network_services::sc_network_service_add_protocol_type(
                self.raw.as_ptr(),
                protocol_type.as_ptr(),
            )
        };
        bridge::bool_result("sc_network_service_add_protocol_type", ok)
    }

    pub fn establish_default_configuration(&self) -> Result<()> {
        let ok = unsafe {
            ffi::network_services::sc_network_service_establish_default_configuration(
                self.raw.as_ptr(),
            )
        };
        bridge::bool_result("sc_network_service_establish_default_configuration", ok)
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::network_services::sc_network_service_get_enabled(self.raw.as_ptr()) != 0 }
    }

    pub fn interface(&self) -> Option<NetworkInterface> {
        unsafe {
            bridge::OwnedHandle::from_raw(ffi::network_services::sc_network_service_copy_interface(
                self.raw.as_ptr(),
            ))
        }
        .map(NetworkInterface::from_owned_handle)
    }

    pub fn name(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_services::sc_network_service_copy_name(self.raw.as_ptr())
        }))
    }

    pub fn service_id(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_services::sc_network_service_copy_service_id(self.raw.as_ptr())
        }))
    }

    pub fn remove(&self) -> Result<()> {
        let ok = unsafe { ffi::network_services::sc_network_service_remove(self.raw.as_ptr()) };
        bridge::bool_result("sc_network_service_remove", ok)
    }

    pub fn remove_protocol_type(&self, protocol_type: &str) -> Result<()> {
        let protocol_type =
            bridge::cstring(protocol_type, "sc_network_service_remove_protocol_type")?;
        let ok = unsafe {
            ffi::network_services::sc_network_service_remove_protocol_type(
                self.raw.as_ptr(),
                protocol_type.as_ptr(),
            )
        };
        bridge::bool_result("sc_network_service_remove_protocol_type", ok)
    }

    pub fn set_enabled(&self, enabled: bool) -> Result<()> {
        let ok = unsafe {
            ffi::network_services::sc_network_service_set_enabled(
                self.raw.as_ptr(),
                u8::from(enabled),
            )
        };
        bridge::bool_result("sc_network_service_set_enabled", ok)
    }

    pub fn set_name(&self, name: Option<&str>) -> Result<()> {
        let name = bridge::optional_cstring(name, "sc_network_service_set_name")?;
        let ok = unsafe {
            ffi::network_services::sc_network_service_set_name(
                self.raw.as_ptr(),
                name.as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        bridge::bool_result("sc_network_service_set_name", ok)
    }

    pub(crate) fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self { raw }
    }

    pub(crate) fn as_ptr(&self) -> bridge::RawHandle {
        self.raw.as_ptr()
    }
}
