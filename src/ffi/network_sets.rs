use std::ffi::c_char;

use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_network_set_get_type_id() -> u64;
    pub(crate) fn sc_network_set_copy_all(raw: Handle) -> Handle;
    pub(crate) fn sc_network_set_copy_current(raw: Handle) -> Handle;
    pub(crate) fn sc_network_set_create(raw: Handle) -> Handle;
    pub(crate) fn sc_network_set_copy(raw: Handle, set_id: *const c_char) -> Handle;
    pub(crate) fn sc_network_set_copy_services(raw: Handle) -> Handle;
    pub(crate) fn sc_network_set_copy_name(raw: Handle) -> Handle;
    pub(crate) fn sc_network_set_copy_set_id(raw: Handle) -> Handle;
    pub(crate) fn sc_network_set_copy_service_order(raw: Handle) -> Handle;
    pub(crate) fn sc_network_set_contains_interface(raw: Handle, interface_raw: Handle) -> u8;
    pub(crate) fn sc_network_set_add_service(raw: Handle, service_raw: Handle) -> u8;
    pub(crate) fn sc_network_set_remove(raw: Handle) -> u8;
    pub(crate) fn sc_network_set_remove_service(raw: Handle, service_raw: Handle) -> u8;
    pub(crate) fn sc_network_set_set_current(raw: Handle) -> u8;
    pub(crate) fn sc_network_set_set_name(raw: Handle, name: *const c_char) -> u8;
    pub(crate) fn sc_network_set_set_service_order(raw: Handle, values: *const *const c_char, count: isize) -> u8;
}
