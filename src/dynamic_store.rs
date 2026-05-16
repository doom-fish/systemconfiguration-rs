use crate::{
    cf::{cfarray_from_strings, cfarray_to_strings, cfstring_from_str, OwnedCFType, PropertyList},
    error::Result,
    ffi, SystemConfigurationError,
};

#[derive(Clone, Debug)]
pub struct DynamicStore {
    raw: OwnedCFType,
}

impl DynamicStore {
    pub fn new(name: &str) -> Result<Self> {
        let name = cfstring_from_str(name)?;
        let raw = unsafe {
            ffi::SCDynamicStoreCreate(
                std::ptr::null(),
                name.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
            )
        };
        let raw = unsafe { OwnedCFType::from_create_rule(raw) }
            .ok_or_else(|| SystemConfigurationError::last("SCDynamicStoreCreate"))?;
        Ok(Self { raw })
    }

    pub fn copy_value(&self, key: &str) -> Result<Option<PropertyList>> {
        let key = cfstring_from_str(key)?;
        let value = unsafe { ffi::SCDynamicStoreCopyValue(self.raw.as_ptr(), key.as_ptr()) };
        Ok(unsafe { PropertyList::from_create_rule(value) })
    }

    pub fn set_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = cfstring_from_str(key)?;
        let ok =
            unsafe { ffi::SCDynamicStoreSetValue(self.raw.as_ptr(), key.as_ptr(), value.as_ptr()) };
        if ok == 0 {
            Err(SystemConfigurationError::last("SCDynamicStoreSetValue"))
        } else {
            Ok(())
        }
    }

    pub fn copy_key_list(&self, pattern: &str) -> Result<Vec<String>> {
        let pattern = cfstring_from_str(pattern)?;
        let keys = unsafe { ffi::SCDynamicStoreCopyKeyList(self.raw.as_ptr(), pattern.as_ptr()) };
        cfarray_to_strings(keys)
    }

    pub fn set_notification_keys<K, P>(&self, keys: &[K], patterns: &[P]) -> Result<()>
    where
        K: AsRef<str>,
        P: AsRef<str>,
    {
        let keys_array = cfarray_from_strings(keys)?;
        let patterns_array = cfarray_from_strings(patterns)?;
        let ok = unsafe {
            ffi::SCDynamicStoreSetNotificationKeys(
                self.raw.as_ptr(),
                keys_array
                    .as_ref()
                    .map_or(std::ptr::null(), OwnedCFType::as_ptr),
                patterns_array
                    .as_ref()
                    .map_or(std::ptr::null(), OwnedCFType::as_ptr),
            )
        };
        if ok == 0 {
            Err(SystemConfigurationError::last(
                "SCDynamicStoreSetNotificationKeys",
            ))
        } else {
            Ok(())
        }
    }
}
