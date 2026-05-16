use crate::{
    bridge::{self, CStringArray},
    error::Result,
    ffi,
    PropertyList, SystemConfigurationError,
};

#[derive(Clone, Debug)]
pub struct DynamicStore {
    raw: bridge::OwnedHandle,
}

impl DynamicStore {
    pub fn new(name: &str) -> Result<Self> {
        Self::create(name, false)
    }

    pub fn new_with_session_keys(name: &str) -> Result<Self> {
        Self::create(name, true)
    }

    fn create(name: &str, use_session_keys: bool) -> Result<Self> {
        let name = bridge::cstring(name, "sc_dynamic_store_create")?;
        let raw = unsafe { ffi::dynamic_store::sc_dynamic_store_create(name.as_ptr(), u8::from(use_session_keys)) };
        let raw = bridge::owned_handle_or_last("sc_dynamic_store_create", raw)?;
        Ok(Self { raw })
    }

    pub fn copy_value(&self, key: &str) -> Result<Option<PropertyList>> {
        let key = bridge::cstring(key, "sc_dynamic_store_copy_value")?;
        let raw = unsafe { ffi::dynamic_store::sc_dynamic_store_copy_value(self.raw.as_ptr(), key.as_ptr()) };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    pub fn copy_multiple<K, P>(&self, keys: &[K], patterns: &[P]) -> Result<Option<PropertyList>>
    where
        K: AsRef<str>,
        P: AsRef<str>,
    {
        let keys = CStringArray::new(keys, "sc_dynamic_store_copy_multiple")?;
        let patterns = CStringArray::new(patterns, "sc_dynamic_store_copy_multiple")?;
        let raw = unsafe {
            ffi::dynamic_store::sc_dynamic_store_copy_multiple(
                self.raw.as_ptr(),
                keys.as_ptr(),
                keys.count(),
                patterns.as_ptr(),
                patterns.count(),
            )
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    pub fn add_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = bridge::cstring(key, "sc_dynamic_store_add_value")?;
        let ok = unsafe {
            ffi::dynamic_store::sc_dynamic_store_add_value(self.raw.as_ptr(), key.as_ptr(), value.as_ptr())
        };
        bridge::bool_result("sc_dynamic_store_add_value", ok)
    }

    pub fn add_temporary_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = bridge::cstring(key, "sc_dynamic_store_add_temporary_value")?;
        let ok = unsafe {
            ffi::dynamic_store::sc_dynamic_store_add_temporary_value(
                self.raw.as_ptr(),
                key.as_ptr(),
                value.as_ptr(),
            )
        };
        bridge::bool_result("sc_dynamic_store_add_temporary_value", ok)
    }

    pub fn set_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = bridge::cstring(key, "sc_dynamic_store_set_value")?;
        let ok = unsafe {
            ffi::dynamic_store::sc_dynamic_store_set_value(self.raw.as_ptr(), key.as_ptr(), value.as_ptr())
        };
        bridge::bool_result("sc_dynamic_store_set_value", ok)
    }

    pub fn remove_value(&self, key: &str) -> Result<()> {
        let key = bridge::cstring(key, "sc_dynamic_store_remove_value")?;
        let ok = unsafe { ffi::dynamic_store::sc_dynamic_store_remove_value(self.raw.as_ptr(), key.as_ptr()) };
        bridge::bool_result("sc_dynamic_store_remove_value", ok)
    }

    pub fn notify_value(&self, key: &str) -> Result<()> {
        let key = bridge::cstring(key, "sc_dynamic_store_notify_value")?;
        let ok = unsafe { ffi::dynamic_store::sc_dynamic_store_notify_value(self.raw.as_ptr(), key.as_ptr()) };
        bridge::bool_result("sc_dynamic_store_notify_value", ok)
    }

    pub fn copy_key_list(&self, pattern: &str) -> Result<Vec<String>> {
        let pattern = bridge::cstring(pattern, "sc_dynamic_store_copy_key_list")?;
        let raw = unsafe { ffi::dynamic_store::sc_dynamic_store_copy_key_list(self.raw.as_ptr(), pattern.as_ptr()) };
        Ok(bridge::take_string_array(raw))
    }

    pub fn set_notification_keys<K, P>(&self, keys: &[K], patterns: &[P]) -> Result<()>
    where
        K: AsRef<str>,
        P: AsRef<str>,
    {
        let keys = CStringArray::new(keys, "sc_dynamic_store_set_notification_keys")?;
        let patterns = CStringArray::new(patterns, "sc_dynamic_store_set_notification_keys")?;
        let ok = unsafe {
            ffi::dynamic_store::sc_dynamic_store_set_notification_keys(
                self.raw.as_ptr(),
                keys.as_ptr(),
                keys.count(),
                patterns.as_ptr(),
                patterns.count(),
            )
        };
        bridge::bool_result("sc_dynamic_store_set_notification_keys", ok)
    }

    pub fn copy_notified_keys(&self) -> Vec<String> {
        let raw = unsafe { ffi::dynamic_store::sc_dynamic_store_copy_notified_keys(self.raw.as_ptr()) };
        bridge::take_string_array(raw)
    }

    pub fn computer_name(&self) -> Option<String> {
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_copy_computer_name(self.raw.as_ptr())
        })
    }

    pub fn local_host_name(&self) -> Option<String> {
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_copy_local_host_name(self.raw.as_ptr())
        })
    }

    pub fn location(&self) -> Option<String> {
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_copy_location(self.raw.as_ptr())
        })
    }

    pub fn proxies(&self) -> Option<PropertyList> {
        let raw = unsafe { ffi::dynamic_store::sc_dynamic_store_copy_proxies(self.raw.as_ptr()) };
        unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle)
    }

    pub fn dhcp_info(&self, service_id: Option<&str>) -> Result<Option<PropertyList>> {
        let service_id = bridge::optional_cstring(service_id, "sc_dynamic_store_copy_dhcp_info")?;
        let raw = unsafe {
            ffi::dynamic_store::sc_dynamic_store_copy_dhcp_info(
                self.raw.as_ptr(),
                service_id.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    pub fn network_global_entity_key(domain: &str, entity: &str) -> Result<String> {
        let domain = bridge::cstring(domain, "sc_dynamic_store_key_create_network_global_entity")?;
        let entity = bridge::cstring(entity, "sc_dynamic_store_key_create_network_global_entity")?;
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_network_global_entity(
                domain.as_ptr(),
                entity.as_ptr(),
            )
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create_network_global_entity",
                "bridge returned null dynamic-store global entity key",
            )
        })
    }

    pub fn network_interface_key(domain: &str) -> Result<String> {
        let domain = bridge::cstring(domain, "sc_dynamic_store_key_create_network_interface")?;
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_network_interface(domain.as_ptr())
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create_network_interface",
                "bridge returned null dynamic-store interface key",
            )
        })
    }

    pub fn network_interface_entity_key(
        domain: &str,
        interface_name: &str,
        entity: Option<&str>,
    ) -> Result<String> {
        let domain = bridge::cstring(domain, "sc_dynamic_store_key_create_network_interface_entity")?;
        let interface_name = bridge::cstring(
            interface_name,
            "sc_dynamic_store_key_create_network_interface_entity",
        )?;
        let entity = bridge::optional_cstring(entity, "sc_dynamic_store_key_create_network_interface_entity")?;
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_network_interface_entity(
                domain.as_ptr(),
                interface_name.as_ptr(),
                entity.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create_network_interface_entity",
                "bridge returned null dynamic-store interface-entity key",
            )
        })
    }

    pub fn network_service_entity_key(
        domain: &str,
        service_id: &str,
        entity: Option<&str>,
    ) -> Result<String> {
        let domain = bridge::cstring(domain, "sc_dynamic_store_key_create_network_service_entity")?;
        let service_id = bridge::cstring(service_id, "sc_dynamic_store_key_create_network_service_entity")?;
        let entity = bridge::optional_cstring(entity, "sc_dynamic_store_key_create_network_service_entity")?;
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_network_service_entity(
                domain.as_ptr(),
                service_id.as_ptr(),
                entity.as_ref().map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create_network_service_entity",
                "bridge returned null dynamic-store service-entity key",
            )
        })
    }

    pub fn computer_name_key() -> Result<String> {
        bridge::take_optional_string(unsafe { ffi::dynamic_store::sc_dynamic_store_key_create_computer_name() })
            .ok_or_else(|| {
                SystemConfigurationError::null(
                    "sc_dynamic_store_key_create_computer_name",
                    "bridge returned null computer-name notification key",
                )
            })
    }

    pub fn console_user_key() -> Result<String> {
        bridge::take_optional_string(unsafe { ffi::dynamic_store::sc_dynamic_store_key_create_console_user() })
            .ok_or_else(|| {
                SystemConfigurationError::null(
                    "sc_dynamic_store_key_create_console_user",
                    "bridge returned null console-user notification key",
                )
            })
    }

    pub fn host_names_key() -> Result<String> {
        bridge::take_optional_string(unsafe { ffi::dynamic_store::sc_dynamic_store_key_create_host_names() })
            .ok_or_else(|| {
                SystemConfigurationError::null(
                    "sc_dynamic_store_key_create_host_names",
                    "bridge returned null host-names notification key",
                )
            })
    }

    pub fn location_key() -> Result<String> {
        bridge::take_optional_string(unsafe { ffi::dynamic_store::sc_dynamic_store_key_create_location() })
            .ok_or_else(|| {
                SystemConfigurationError::null(
                    "sc_dynamic_store_key_create_location",
                    "bridge returned null location notification key",
                )
            })
    }

    pub fn proxies_key() -> Result<String> {
        bridge::take_optional_string(unsafe { ffi::dynamic_store::sc_dynamic_store_key_create_proxies() })
            .ok_or_else(|| {
                SystemConfigurationError::null(
                    "sc_dynamic_store_key_create_proxies",
                    "bridge returned null proxies notification key",
                )
            })
    }
}
