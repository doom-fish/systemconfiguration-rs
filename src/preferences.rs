use crate::{
    cf::{
        cfarray_to_owned, cfstring_from_str, optional_cfstring_to_string, OwnedCFType, PropertyList,
    },
    error::Result,
    ffi, SystemConfigurationError,
};

#[derive(Clone, Debug)]
pub struct Preferences {
    raw: OwnedCFType,
}

impl Preferences {
    pub fn new(name: &str, prefs_id: Option<&str>) -> Result<Self> {
        let name = cfstring_from_str(name)?;
        let prefs_id = prefs_id.map(cfstring_from_str).transpose()?;
        let raw = unsafe {
            ffi::SCPreferencesCreate(
                std::ptr::null(),
                name.as_ptr(),
                prefs_id
                    .as_ref()
                    .map_or(std::ptr::null(), OwnedCFType::as_ptr),
            )
        };
        let raw = unsafe { OwnedCFType::from_create_rule(raw) }
            .ok_or_else(|| SystemConfigurationError::last("SCPreferencesCreate"))?;
        Ok(Self { raw })
    }

    pub fn lock(&self, wait: bool) -> Result<()> {
        let ok = unsafe { ffi::SCPreferencesLock(self.raw.as_ptr(), u8::from(wait)) };
        if ok == 0 {
            Err(SystemConfigurationError::last("SCPreferencesLock"))
        } else {
            Ok(())
        }
    }

    pub fn set_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = cfstring_from_str(key)?;
        let ok =
            unsafe { ffi::SCPreferencesSetValue(self.raw.as_ptr(), key.as_ptr(), value.as_ptr()) };
        if ok == 0 {
            Err(SystemConfigurationError::last("SCPreferencesSetValue"))
        } else {
            Ok(())
        }
    }

    pub fn commit_changes(&self) -> Result<()> {
        let ok = unsafe { ffi::SCPreferencesCommitChanges(self.raw.as_ptr()) };
        if ok == 0 {
            Err(SystemConfigurationError::last("SCPreferencesCommitChanges"))
        } else {
            Ok(())
        }
    }

    pub fn network_services(&self) -> Vec<NetworkService> {
        let services = unsafe { ffi::SCNetworkServiceCopyAll(self.raw.as_ptr()) };
        cfarray_to_owned(services)
            .into_iter()
            .map(|raw| NetworkService { raw })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct NetworkService {
    raw: OwnedCFType,
}

impl NetworkService {
    pub fn service_id(&self) -> Result<Option<String>> {
        let service_id = unsafe { ffi::SCNetworkServiceGetServiceID(self.raw.as_ptr()) };
        optional_cfstring_to_string(service_id)
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { ffi::SCNetworkServiceGetEnabled(self.raw.as_ptr()) != 0 }
    }
}
