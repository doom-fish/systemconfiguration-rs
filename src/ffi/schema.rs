use super::core::Handle;

unsafe extern "C" {
    pub(crate) fn sc_schema_copy_catalog() -> Handle;
}
