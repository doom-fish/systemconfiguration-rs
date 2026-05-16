use crate::{
    cf::{cfarray_to_owned, optional_cfstring_to_string, OwnedCFType},
    error::Result,
    ffi, SystemConfigurationError,
};

#[derive(Clone, Debug)]
pub struct NetworkInterface {
    raw: OwnedCFType,
}

impl NetworkInterface {
    pub fn copy_all() -> Result<Vec<Self>> {
        let interfaces = unsafe { ffi::SCNetworkInterfaceCopyAll() };
        if interfaces.is_null() {
            return Err(SystemConfigurationError::last("SCNetworkInterfaceCopyAll"));
        }

        Ok(cfarray_to_owned(interfaces)
            .into_iter()
            .map(|raw| Self { raw })
            .collect())
    }

    pub fn bsd_name(&self) -> Result<Option<String>> {
        let value = unsafe { ffi::SCNetworkInterfaceGetBSDName(self.raw.as_ptr()) };
        optional_cfstring_to_string(value)
    }

    pub fn interface_type(&self) -> Result<Option<String>> {
        let value = unsafe { ffi::SCNetworkInterfaceGetInterfaceType(self.raw.as_ptr()) };
        optional_cfstring_to_string(value)
    }

    pub fn localized_display_name(&self) -> Result<Option<String>> {
        let value = unsafe { ffi::SCNetworkInterfaceGetLocalizedDisplayName(self.raw.as_ptr()) };
        optional_cfstring_to_string(value)
    }

    pub fn hardware_address_string(&self) -> Result<Option<String>> {
        let value = unsafe { ffi::SCNetworkInterfaceGetHardwareAddressString(self.raw.as_ptr()) };
        optional_cfstring_to_string(value)
    }
}
