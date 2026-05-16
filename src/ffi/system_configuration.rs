use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_copy_last_error_json() -> Handle;
    pub(crate) fn sc_system_configuration_error_domain() -> Handle;
}
