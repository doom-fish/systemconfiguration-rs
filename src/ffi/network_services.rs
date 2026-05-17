use std::ffi::c_char;

use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_network_service_get_type_id() -> u64;
    pub(crate) fn sc_network_service_copy_all(raw: Handle) -> Handle;
    pub(crate) fn sc_network_service_create(prefs_raw: Handle, interface_raw: Handle) -> Handle;
    pub(crate) fn sc_network_service_copy(raw: Handle, service_id: *const c_char) -> Handle;
    pub(crate) fn sc_network_service_copy_protocols(raw: Handle) -> Handle;
    pub(crate) fn sc_network_service_copy_protocol(
        raw: Handle,
        protocol_type: *const c_char,
    ) -> Handle;
    pub(crate) fn sc_network_service_add_protocol_type(
        raw: Handle,
        protocol_type: *const c_char,
    ) -> u8;
    pub(crate) fn sc_network_service_establish_default_configuration(raw: Handle) -> u8;
    pub(crate) fn sc_network_service_get_enabled(raw: Handle) -> u8;
    pub(crate) fn sc_network_service_copy_interface(raw: Handle) -> Handle;
    pub(crate) fn sc_network_service_copy_name(raw: Handle) -> Handle;
    pub(crate) fn sc_network_service_copy_service_id(raw: Handle) -> Handle;
    pub(crate) fn sc_network_service_remove(raw: Handle) -> u8;
    pub(crate) fn sc_network_service_remove_protocol_type(
        raw: Handle,
        protocol_type: *const c_char,
    ) -> u8;
    pub(crate) fn sc_network_service_set_enabled(raw: Handle, enabled: u8) -> u8;
    pub(crate) fn sc_network_service_set_name(raw: Handle, name: *const c_char) -> u8;
}
