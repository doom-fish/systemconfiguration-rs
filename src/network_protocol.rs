use crate::{bridge, error::Result, ffi, PropertyList};

#[derive(Clone, Debug)]
pub struct NetworkProtocol {
    raw: bridge::OwnedHandle,
}

impl NetworkProtocol {
    pub fn type_id() -> u64 {
        unsafe { ffi::network_protocol::sc_network_protocol_get_type_id() }
    }

    pub fn configuration(&self) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_protocol::sc_network_protocol_copy_configuration(self.raw.as_ptr()),
            )
        }
        .map(PropertyList::from_owned_handle)
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::network_protocol::sc_network_protocol_get_enabled(self.raw.as_ptr()) != 0 }
    }

    pub fn protocol_type(&self) -> Option<String> {
        bridge::take_optional_string(unsafe {
            ffi::network_protocol::sc_network_protocol_copy_protocol_type(self.raw.as_ptr())
        })
    }

    pub fn set_configuration(&self, value: &PropertyList) -> Result<()> {
        let ok = unsafe {
            ffi::network_protocol::sc_network_protocol_set_configuration(
                self.raw.as_ptr(),
                value.as_ptr(),
            )
        };
        bridge::bool_result("sc_network_protocol_set_configuration", ok)
    }

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
