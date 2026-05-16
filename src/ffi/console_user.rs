use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_console_user_copy_current() -> Handle;
    pub(crate) fn sc_console_user_copy_name(raw: Handle) -> Handle;
    pub(crate) fn sc_console_user_get_uid(raw: Handle) -> u32;
    pub(crate) fn sc_console_user_get_gid(raw: Handle) -> u32;
}
