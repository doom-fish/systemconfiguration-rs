use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_network_configuration_copy_overview() -> Handle;
}
