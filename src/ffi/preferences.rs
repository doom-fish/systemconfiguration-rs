use std::ffi::c_char;

use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_preferences_create(name: *const c_char, prefs_id: *const c_char) -> Handle;
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
    pub(crate) fn sc_preferences_path_create_unique_child(raw: Handle, prefix: *const c_char) -> Handle;
    pub(crate) fn sc_preferences_path_get_value(raw: Handle, path: *const c_char) -> Handle;
    pub(crate) fn sc_preferences_path_get_link(raw: Handle, path: *const c_char) -> Handle;
    pub(crate) fn sc_preferences_path_set_value(raw: Handle, path: *const c_char, value: Handle) -> u8;
    pub(crate) fn sc_preferences_path_set_link(raw: Handle, path: *const c_char, link: *const c_char) -> u8;
    pub(crate) fn sc_preferences_path_remove_value(raw: Handle, path: *const c_char) -> u8;
    pub(crate) fn sc_preferences_set_computer_name(raw: Handle, name: *const c_char) -> u8;
    pub(crate) fn sc_preferences_set_local_host_name(raw: Handle, name: *const c_char) -> u8;
}
