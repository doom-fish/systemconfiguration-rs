use std::ffi::c_char;

use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_captive_network_copy_supported_interfaces() -> Handle;
    pub(crate) fn sc_captive_network_set_supported_ssids(values: *const *const c_char, count: isize) -> u8;
    pub(crate) fn sc_captive_network_mark_portal_online(interface_name: *const c_char) -> u8;
    pub(crate) fn sc_captive_network_mark_portal_offline(interface_name: *const c_char) -> u8;
}
