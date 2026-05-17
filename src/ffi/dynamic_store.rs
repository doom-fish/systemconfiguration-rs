use std::ffi::{c_char, c_void};

use super::core::Handle;

pub(crate) type DynamicStoreCallback =
    Option<unsafe extern "C" fn(changed_keys_raw: Handle, info: *mut c_void)>;

unsafe extern "C" {
    pub(crate) fn sc_dynamic_store_get_type_id() -> u64;
    pub(crate) fn sc_dynamic_store_create(name: *const c_char, use_session_keys: u8) -> Handle;
    pub(crate) fn sc_dynamic_store_create_with_options(
        name: *const c_char,
        options: Handle,
    ) -> Handle;
    pub(crate) fn sc_dynamic_store_create_with_callback(
        name: *const c_char,
        use_session_keys: u8,
        callback: DynamicStoreCallback,
        info: *mut c_void,
    ) -> Handle;
    pub(crate) fn sc_dynamic_store_create_with_options_and_callback(
        name: *const c_char,
        options: Handle,
        callback: DynamicStoreCallback,
        info: *mut c_void,
    ) -> Handle;
    pub(crate) fn sc_dynamic_store_copy_key_list(raw: Handle, pattern: *const c_char) -> Handle;
    pub(crate) fn sc_dynamic_store_copy_value(raw: Handle, key: *const c_char) -> Handle;
    pub(crate) fn sc_dynamic_store_copy_multiple(
        raw: Handle,
        keys: *const *const c_char,
        key_count: isize,
        patterns: *const *const c_char,
        pattern_count: isize,
    ) -> Handle;
    pub(crate) fn sc_dynamic_store_add_value(raw: Handle, key: *const c_char, value: Handle) -> u8;
    pub(crate) fn sc_dynamic_store_add_temporary_value(
        raw: Handle,
        key: *const c_char,
        value: Handle,
    ) -> u8;
    pub(crate) fn sc_dynamic_store_set_value(raw: Handle, key: *const c_char, value: Handle) -> u8;
    pub(crate) fn sc_dynamic_store_remove_value(raw: Handle, key: *const c_char) -> u8;
    pub(crate) fn sc_dynamic_store_notify_value(raw: Handle, key: *const c_char) -> u8;
    pub(crate) fn sc_dynamic_store_set_notification_keys(
        raw: Handle,
        keys: *const *const c_char,
        key_count: isize,
        patterns: *const *const c_char,
        pattern_count: isize,
    ) -> u8;
    pub(crate) fn sc_dynamic_store_set_multiple(
        raw: Handle,
        keys_to_set: Handle,
        keys_to_remove: *const *const c_char,
        remove_count: isize,
        keys_to_notify: *const *const c_char,
        notify_count: isize,
    ) -> u8;
    pub(crate) fn sc_dynamic_store_create_run_loop_source(raw: Handle, order: isize) -> Handle;
    pub(crate) fn sc_run_loop_source_schedule_current_default_mode(raw: Handle) -> u8;
    pub(crate) fn sc_run_loop_source_unschedule_current_default_mode(raw: Handle) -> u8;
    pub(crate) fn sc_dynamic_store_set_dispatch_queue_global(raw: Handle) -> u8;
    pub(crate) fn sc_dynamic_store_clear_dispatch_queue(raw: Handle) -> u8;
    pub(crate) fn sc_dynamic_store_copy_notified_keys(raw: Handle) -> Handle;
    pub(crate) fn sc_dynamic_store_copy_computer_name(raw: Handle) -> Handle;
    pub(crate) fn sc_dynamic_store_copy_local_host_name(raw: Handle) -> Handle;
    pub(crate) fn sc_dynamic_store_copy_location(raw: Handle) -> Handle;
    pub(crate) fn sc_dynamic_store_copy_proxies(raw: Handle) -> Handle;
    pub(crate) fn sc_dynamic_store_copy_dhcp_info(raw: Handle, service_id: *const c_char)
        -> Handle;
    pub(crate) fn sc_dhcp_info_copy_option_data(info_raw: Handle, code: u8) -> Handle;
    pub(crate) fn sc_dhcp_info_copy_lease_start_time(info_raw: Handle) -> Handle;
    pub(crate) fn sc_dhcp_info_copy_lease_expiration_time(info_raw: Handle) -> Handle;
    pub(crate) fn sc_dynamic_store_key_create(
        format: *const c_char,
        arguments: *const *const c_char,
        count: isize,
    ) -> Handle;
    pub(crate) fn sc_dynamic_store_key_create_network_global_entity(
        domain: *const c_char,
        entity: *const c_char,
    ) -> Handle;
    pub(crate) fn sc_dynamic_store_key_create_network_interface(domain: *const c_char) -> Handle;
    pub(crate) fn sc_dynamic_store_key_create_network_interface_entity(
        domain: *const c_char,
        interface_name: *const c_char,
        entity: *const c_char,
    ) -> Handle;
    pub(crate) fn sc_dynamic_store_key_create_network_service_entity(
        domain: *const c_char,
        service_id: *const c_char,
        entity: *const c_char,
    ) -> Handle;
    pub(crate) fn sc_dynamic_store_key_create_computer_name() -> Handle;
    pub(crate) fn sc_dynamic_store_key_create_console_user() -> Handle;
    pub(crate) fn sc_dynamic_store_key_create_host_names() -> Handle;
    pub(crate) fn sc_dynamic_store_key_create_location() -> Handle;
    pub(crate) fn sc_dynamic_store_key_create_proxies() -> Handle;
}
