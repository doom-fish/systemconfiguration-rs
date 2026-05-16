use std::ffi::c_char;

use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_network_interface_copy_all() -> Handle;
    pub(crate) fn sc_network_interface_copy_supported_interface_types(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_copy_supported_protocol_types(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_create_with_interface(raw: Handle, interface_type: *const c_char) -> Handle;
    pub(crate) fn sc_network_interface_copy_bsd_name(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_copy_configuration(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_copy_extended_configuration(raw: Handle, extended_type: *const c_char) -> Handle;
    pub(crate) fn sc_network_interface_copy_hardware_address(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_copy_underlying_interface(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_copy_interface_type(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_copy_localized_display_name(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_copy_mtu_info(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_force_configuration_refresh(raw: Handle) -> u8;
}
