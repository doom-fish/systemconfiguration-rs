use std::ffi::{c_char, c_void};

pub(crate) type Handle = *mut c_void;

unsafe extern "C" {
    pub(crate) fn sc_handle_retain(raw: Handle) -> Handle;
    pub(crate) fn sc_handle_release(raw: Handle);

    pub(crate) fn sc_last_error_code() -> i32;
    pub(crate) fn sc_last_error_message() -> Handle;

    pub(crate) fn sc_string_len(raw: Handle) -> isize;
    pub(crate) fn sc_string_copy_utf8(raw: Handle, buffer: *mut u8, capacity: isize) -> isize;

    pub(crate) fn sc_array_count(raw: Handle) -> isize;
    pub(crate) fn sc_array_get(raw: Handle, index: isize) -> Handle;

    pub(crate) fn sc_property_list_from_string(value: *const c_char) -> Handle;
    pub(crate) fn sc_property_list_from_bool(value: u8) -> Handle;
    pub(crate) fn sc_property_list_from_json(value: *const c_char) -> Handle;
    pub(crate) fn sc_property_list_copy_description(raw: Handle) -> Handle;
}
