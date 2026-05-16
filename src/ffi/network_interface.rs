use std::ffi::c_char;

use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_network_interface_get_type_id() -> u64;
    pub(crate) fn sc_network_interface_copy_all() -> Handle;
    pub(crate) fn sc_network_interface_copy_ipv4() -> Handle;
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
    pub(crate) fn sc_network_interface_copy_media_options_current(raw: Handle, filter: u8) -> Handle;
    pub(crate) fn sc_network_interface_copy_media_options_active(raw: Handle, filter: u8) -> Handle;
    pub(crate) fn sc_network_interface_copy_media_options_available(raw: Handle, filter: u8) -> Handle;
    pub(crate) fn sc_network_interface_copy_media_subtypes(available_raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_copy_media_subtype_options(
        available_raw: Handle,
        subtype: *const c_char,
    ) -> Handle;
    pub(crate) fn sc_network_interface_copy_mtu_info(raw: Handle) -> Handle;
    pub(crate) fn sc_network_interface_set_configuration(raw: Handle, config_raw: Handle) -> u8;
    pub(crate) fn sc_network_interface_set_extended_configuration(
        raw: Handle,
        extended_type: *const c_char,
        config_raw: Handle,
    ) -> u8;
    pub(crate) fn sc_network_interface_set_media_options(
        raw: Handle,
        subtype: *const c_char,
        options: *const *const c_char,
        count: isize,
    ) -> u8;
    pub(crate) fn sc_network_interface_set_mtu(raw: Handle, mtu: i32) -> u8;
    pub(crate) fn sc_network_interface_force_configuration_refresh(raw: Handle) -> u8;

    pub(crate) fn sc_bond_interface_copy_all(prefs_raw: Handle) -> Handle;
    pub(crate) fn sc_bond_interface_copy_available_member_interfaces(prefs_raw: Handle) -> Handle;
    pub(crate) fn sc_bond_interface_create(prefs_raw: Handle) -> Handle;
    pub(crate) fn sc_bond_interface_remove(raw: Handle) -> u8;
    pub(crate) fn sc_bond_interface_copy_member_interfaces(raw: Handle) -> Handle;
    pub(crate) fn sc_bond_interface_copy_options(raw: Handle) -> Handle;
    pub(crate) fn sc_bond_interface_set_member_interfaces(
        raw: Handle,
        members: *const Handle,
        count: isize,
    ) -> u8;
    pub(crate) fn sc_bond_interface_set_localized_display_name(raw: Handle, name: *const c_char) -> u8;
    pub(crate) fn sc_bond_interface_set_options(raw: Handle, options_raw: Handle) -> u8;
    pub(crate) fn sc_bond_interface_copy_status(raw: Handle) -> Handle;
    pub(crate) fn sc_bond_status_get_type_id() -> u64;
    pub(crate) fn sc_bond_status_copy_device_aggregation_status_key() -> Handle;
    pub(crate) fn sc_bond_status_copy_device_collecting_key() -> Handle;
    pub(crate) fn sc_bond_status_copy_device_distributing_key() -> Handle;
    pub(crate) fn sc_bond_status_copy_member_interfaces(raw: Handle) -> Handle;
    pub(crate) fn sc_bond_status_copy_interface_status(raw: Handle, interface_raw: Handle) -> Handle;

    pub(crate) fn sc_vlan_interface_copy_all(prefs_raw: Handle) -> Handle;
    pub(crate) fn sc_vlan_interface_copy_available_physical_interfaces() -> Handle;
    pub(crate) fn sc_vlan_interface_create(prefs_raw: Handle, physical_raw: Handle, tag: i32) -> Handle;
    pub(crate) fn sc_vlan_interface_remove(raw: Handle) -> u8;
    pub(crate) fn sc_vlan_interface_copy_physical_interface(raw: Handle) -> Handle;
    pub(crate) fn sc_vlan_interface_get_tag(raw: Handle, out_tag: *mut i32) -> u8;
    pub(crate) fn sc_vlan_interface_copy_options(raw: Handle) -> Handle;
    pub(crate) fn sc_vlan_interface_set_physical_interface_and_tag(
        raw: Handle,
        physical_raw: Handle,
        tag: i32,
    ) -> u8;
    pub(crate) fn sc_vlan_interface_set_localized_display_name(raw: Handle, name: *const c_char) -> u8;
    pub(crate) fn sc_vlan_interface_set_options(raw: Handle, options_raw: Handle) -> u8;
}
