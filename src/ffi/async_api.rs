use std::ffi::{c_char, c_void};

pub(crate) type AsyncEventCallback =
    Option<unsafe extern "C" fn(kind: i32, payload: *mut c_void, ctx: *mut c_void)>;

unsafe extern "C" {
    pub(crate) fn sc_dynamic_store_notification_subscribe(
        name: *const c_char,
        keys: *const *const c_char,
        key_count: isize,
        patterns: *const *const c_char,
        pattern_count: isize,
        on_event: AsyncEventCallback,
        ctx: *mut c_void,
    ) -> *mut c_void;

    pub(crate) fn sc_dynamic_store_notification_unsubscribe(handle: *mut c_void);

    pub(crate) fn sc_reachability_notification_subscribe(
        name: *const c_char,
        on_event: AsyncEventCallback,
        ctx: *mut c_void,
    ) -> *mut c_void;

    pub(crate) fn sc_reachability_notification_unsubscribe(handle: *mut c_void);

    pub(crate) fn sc_preferences_notification_subscribe(
        name: *const c_char,
        on_event: AsyncEventCallback,
        ctx: *mut c_void,
    ) -> *mut c_void;

    pub(crate) fn sc_preferences_notification_unsubscribe(handle: *mut c_void);
}
