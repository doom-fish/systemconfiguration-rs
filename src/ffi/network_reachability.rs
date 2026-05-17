use std::ffi::c_void;

use super::core::Handle;

pub(crate) type ReachabilityCallback = Option<unsafe extern "C" fn(flags: u32, info: *mut c_void)>;

unsafe extern "C" {
    pub(crate) fn sc_reachability_get_type_id() -> u64;
    pub(crate) fn sc_reachability_create_with_name(name: *const libc::c_char) -> Handle;
    pub(crate) fn sc_reachability_create_with_address(bytes: *const u8, count: isize) -> Handle;
    pub(crate) fn sc_reachability_create_with_address_pair(
        local_bytes: *const u8,
        local_count: isize,
        remote_bytes: *const u8,
        remote_count: isize,
    ) -> Handle;
    pub(crate) fn sc_reachability_get_flags(raw: Handle, out_flags: *mut u32) -> u8;
    pub(crate) fn sc_reachability_set_callback(
        raw: Handle,
        callback: ReachabilityCallback,
        info: *mut c_void,
    ) -> u8;
    pub(crate) fn sc_reachability_schedule_with_run_loop_current(raw: Handle) -> u8;
    pub(crate) fn sc_reachability_unschedule_from_run_loop_current(raw: Handle) -> u8;
    pub(crate) fn sc_reachability_set_dispatch_queue_global(raw: Handle) -> u8;
    pub(crate) fn sc_reachability_clear_dispatch_queue(raw: Handle) -> u8;
}
