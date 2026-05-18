use crate::{bridge, error::Result, ffi, PropertyList};

#[derive(Clone, Debug)]
/// Wraps `SCNetworkProtocolRef`.
pub struct NetworkProtocol {
    raw: bridge::OwnedHandle,
}

impl NetworkProtocol {
    /// Wraps `SCNetworkProtocolGetTypeID`.
    pub fn type_id() -> u64 {
        unsafe { ffi::network_protocol::sc_network_protocol_get_type_id() }
    }

    /// Wraps `SCNetworkProtocolCopyConfiguration`.
    pub fn configuration(&self) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_protocol::sc_network_protocol_copy_configuration(self.raw.as_ptr()),
            )
        }
        .map(PropertyList::from_owned_handle)
    }

    /// Wraps `SCNetworkProtocolGetEnabled`.
    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::network_protocol::sc_network_protocol_get_enabled(self.raw.as_ptr()) != 0 }
    }

    /// Wraps `SCNetworkProtocolCopyProtocolType`.
    pub fn protocol_type(&self) -> Option<String> {
        bridge::take_optional_string(unsafe {
            ffi::network_protocol::sc_network_protocol_copy_protocol_type(self.raw.as_ptr())
        })
    }

    /// Wraps `SCNetworkProtocolSetConfiguration`.
    pub fn set_configuration(&self, value: &PropertyList) -> Result<()> {
        let ok = unsafe {
            ffi::network_protocol::sc_network_protocol_set_configuration(
                self.raw.as_ptr(),
                value.as_ptr(),
            )
        };
        bridge::bool_result("sc_network_protocol_set_configuration", ok)
    }

    /// Wraps `SCNetworkProtocolSetEnabled`.
    pub fn set_enabled(&self, enabled: bool) -> Result<()> {
        let ok = unsafe {
            ffi::network_protocol::sc_network_protocol_set_enabled(
                self.raw.as_ptr(),
                u8::from(enabled),
            )
        };
        bridge::bool_result("sc_network_protocol_set_enabled", ok)
    }

    pub(crate) fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self { raw }
    }
}
