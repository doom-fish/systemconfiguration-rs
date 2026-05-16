use std::ffi::{c_char, c_void};

use super::core::Handle;

pub(crate) type NetworkConnectionCallback = Option<unsafe extern "C" fn(status: i32, info: *mut c_void)>;

unsafe extern "C" {
    pub(crate) fn sc_network_connection_get_type_id() -> u64;
    pub(crate) fn sc_network_connection_create_with_service_id(
        service_id: *const c_char,
        callback: NetworkConnectionCallback,
        info: *mut c_void,
    ) -> Handle;
    pub(crate) fn sc_network_connection_copy_user_preferences_service_id() -> Handle;
    pub(crate) fn sc_network_connection_copy_user_preferences_user_options() -> Handle;
    pub(crate) fn sc_network_connection_copy_service_id(raw: Handle) -> Handle;
    pub(crate) fn sc_network_connection_get_status(raw: Handle) -> i32;
    pub(crate) fn sc_network_connection_copy_extended_status(raw: Handle) -> Handle;
    pub(crate) fn sc_network_connection_copy_statistics(raw: Handle) -> Handle;
    pub(crate) fn sc_network_connection_copy_user_options(raw: Handle) -> Handle;
    pub(crate) fn sc_network_connection_start(raw: Handle, user_options_raw: Handle, linger: u8) -> u8;
    pub(crate) fn sc_network_connection_stop(raw: Handle, force_disconnect: u8) -> u8;
    pub(crate) fn sc_network_connection_schedule_with_run_loop_current(raw: Handle) -> u8;
    pub(crate) fn sc_network_connection_unschedule_from_run_loop_current(raw: Handle) -> u8;
    pub(crate) fn sc_network_connection_set_dispatch_queue_global(raw: Handle) -> u8;
    pub(crate) fn sc_network_connection_clear_dispatch_queue(raw: Handle) -> u8;
}
