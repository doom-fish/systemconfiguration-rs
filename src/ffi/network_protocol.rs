use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_network_protocol_get_type_id() -> u64;
    pub(crate) fn sc_network_protocol_copy_configuration(raw: Handle) -> Handle;
    pub(crate) fn sc_network_protocol_get_enabled(raw: Handle) -> u8;
    pub(crate) fn sc_network_protocol_copy_protocol_type(raw: Handle) -> Handle;
    pub(crate) fn sc_network_protocol_set_configuration(raw: Handle, value: Handle) -> u8;
    pub(crate) fn sc_network_protocol_set_enabled(raw: Handle, enabled: u8) -> u8;
}
