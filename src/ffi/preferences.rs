use std::ffi::{c_char, c_void};

use super::core::Handle;

pub(crate) type PreferencesCallback = Option<unsafe extern "C" fn(notification_type: u32, info: *mut c_void)>;

unsafe extern "C" {
    pub(crate) fn sc_preferences_get_type_id() -> u64;
    pub(crate) fn sc_preferences_create(name: *const c_char, prefs_id: *const c_char) -> Handle;
    pub(crate) fn sc_preferences_create_with_authorization(
        name: *const c_char,
        prefs_id: *const c_char,
        authorization_raw: Handle,
    ) -> Handle;
    pub(crate) fn sc_preferences_lock(raw: Handle, wait: u8) -> u8;
    pub(crate) fn sc_preferences_commit_changes(raw: Handle) -> u8;
    pub(crate) fn sc_preferences_apply_changes(raw: Handle) -> u8;
    pub(crate) fn sc_preferences_unlock(raw: Handle) -> u8;
    pub(crate) fn sc_preferences_synchronize(raw: Handle);
    pub(crate) fn sc_preferences_copy_signature(raw: Handle) -> Handle;
    pub(crate) fn sc_preferences_copy_key_list(raw: Handle) -> Handle;
    pub(crate) fn sc_preferences_get_value(raw: Handle, key: *const c_char) -> Handle;
    pub(crate) fn sc_preferences_add_value(raw: Handle, key: *const c_char, value: Handle) -> u8;
    pub(crate) fn sc_preferences_set_value(raw: Handle, key: *const c_char, value: Handle) -> u8;
    pub(crate) fn sc_preferences_remove_value(raw: Handle, key: *const c_char) -> u8;
    pub(crate) fn sc_preferences_set_callback(
        raw: Handle,
        callback: PreferencesCallback,
        info: *mut c_void,
    ) -> u8;
    pub(crate) fn sc_preferences_schedule_with_run_loop_current(raw: Handle) -> u8;
    pub(crate) fn sc_preferences_unschedule_from_run_loop_current(raw: Handle) -> u8;
    pub(crate) fn sc_preferences_set_dispatch_queue_global(raw: Handle) -> u8;
    pub(crate) fn sc_preferences_clear_dispatch_queue(raw: Handle) -> u8;
    pub(crate) fn sc_preferences_path_create_unique_child(raw: Handle, prefix: *const c_char) -> Handle;
    pub(crate) fn sc_preferences_path_get_value(raw: Handle, path: *const c_char) -> Handle;
    pub(crate) fn sc_preferences_path_get_link(raw: Handle, path: *const c_char) -> Handle;
    pub(crate) fn sc_preferences_path_set_value(raw: Handle, path: *const c_char, value: Handle) -> u8;
    pub(crate) fn sc_preferences_path_set_link(raw: Handle, path: *const c_char, link: *const c_char) -> u8;
    pub(crate) fn sc_preferences_path_remove_value(raw: Handle, path: *const c_char) -> u8;
    pub(crate) fn sc_preferences_set_computer_name(raw: Handle, name: *const c_char) -> u8;
    pub(crate) fn sc_preferences_set_local_host_name(raw: Handle, name: *const c_char) -> u8;
}
