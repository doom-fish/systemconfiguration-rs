use std::{
    ffi::c_void,
    panic::AssertUnwindSafe,
    sync::{Arc, Mutex},
};

use crate::{
    bridge::{self, CStringArray},
    error::Result,
    ffi, PropertyList, SystemConfigurationError,
};

struct CallbackState {
    callback: Box<dyn FnMut(Vec<String>) + Send>,
}

unsafe extern "C" fn dynamic_store_callback(
    changed_keys_raw: bridge::RawHandle,
    info: *mut c_void,
) {
    if info.is_null() {
        return;
    }

    // SAFETY: `info` is `Arc::as_ptr(state).cast_mut().cast::<c_void>()` kept
    // alive by `DynamicStore::_callback` for the entire lifetime of the store.
    // This callback is only invoked while the store is alive.
    let mutex = unsafe { &*info.cast::<Mutex<CallbackState>>() };
    if let Ok(mut state) = mutex.lock() {
        let keys = bridge::take_string_array(changed_keys_raw);
        // Catch panics: unwinding across the Swift/C FFI boundary is UB.
        let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
            (state.callback)(keys);
        }));
    }
}

#[derive(Clone)]
pub struct DynamicStore {
    raw: bridge::OwnedHandle,
    _callback: Option<Arc<Mutex<CallbackState>>>,
}

impl std::fmt::Debug for DynamicStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicStore").finish_non_exhaustive()
    }
}

#[derive(Clone)]
pub struct DynamicStoreRunLoopSource {
    raw: bridge::OwnedHandle,
}

impl std::fmt::Debug for DynamicStoreRunLoopSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicStoreRunLoopSource")
            .finish_non_exhaustive()
    }
}

impl DynamicStore {
    pub fn type_id() -> u64 {
        unsafe { ffi::dynamic_store::sc_dynamic_store_get_type_id() }
    }

    pub fn new(name: &str) -> Result<Self> {
        Self::create(name, None, false, None)
    }

    pub fn new_with_session_keys(name: &str) -> Result<Self> {
        Self::create(name, None, true, None)
    }

    /// `options` must encode a dictionary accepted by `SCDynamicStoreCreateWithOptions`.
    /// Use `new_with_session_keys` for the common session-keys configuration.
    pub fn new_with_options(name: &str, options: &PropertyList) -> Result<Self> {
        Self::create(name, Some(options), false, None)
    }

    pub fn new_with_callback<F>(name: &str, callback: F) -> Result<Self>
    where
        F: FnMut(Vec<String>) + Send + 'static,
    {
        let callback = Arc::new(Mutex::new(CallbackState {
            callback: Box::new(callback),
        }));
        Self::create(name, None, false, Some(callback))
    }

    /// `options` must encode a dictionary accepted by `SCDynamicStoreCreateWithOptions`.
    /// Use `new_with_session_keys_and_callback` for the common session-keys configuration.
    pub fn new_with_options_and_callback<F>(
        name: &str,
        options: &PropertyList,
        callback: F,
    ) -> Result<Self>
    where
        F: FnMut(Vec<String>) + Send + 'static,
    {
        let callback = Arc::new(Mutex::new(CallbackState {
            callback: Box::new(callback),
        }));
        Self::create(name, Some(options), false, Some(callback))
    }

    pub fn new_with_session_keys_and_callback<F>(name: &str, callback: F) -> Result<Self>
    where
        F: FnMut(Vec<String>) + Send + 'static,
    {
        let callback = Arc::new(Mutex::new(CallbackState {
            callback: Box::new(callback),
        }));
        Self::create(name, None, true, Some(callback))
    }

    fn create(
        name: &str,
        options: Option<&PropertyList>,
        use_session_keys: bool,
        callback: Option<Arc<Mutex<CallbackState>>>,
    ) -> Result<Self> {
        let function = match (options.is_some(), callback.is_some()) {
            (false, false) => "sc_dynamic_store_create",
            (false, true) => "sc_dynamic_store_create_with_callback",
            (true, false) => "sc_dynamic_store_create_with_options",
            (true, true) => "sc_dynamic_store_create_with_options_and_callback",
        };
        let name = bridge::cstring(name, function)?;
        let raw = unsafe {
            match (options, callback.as_ref()) {
                (Some(options), None) => ffi::dynamic_store::sc_dynamic_store_create_with_options(
                    name.as_ptr(),
                    options.as_ptr(),
                ),
                (Some(options), Some(state)) => {
                    ffi::dynamic_store::sc_dynamic_store_create_with_options_and_callback(
                        name.as_ptr(),
                        options.as_ptr(),
                        Some(dynamic_store_callback),
                        Arc::as_ptr(state).cast_mut().cast::<c_void>(),
                    )
                }
                (None, None) => ffi::dynamic_store::sc_dynamic_store_create(
                    name.as_ptr(),
                    u8::from(use_session_keys),
                ),
                (None, Some(state)) => ffi::dynamic_store::sc_dynamic_store_create_with_callback(
                    name.as_ptr(),
                    u8::from(use_session_keys),
                    Some(dynamic_store_callback),
                    Arc::as_ptr(state).cast_mut().cast::<c_void>(),
                ),
            }
        };
        let raw = bridge::owned_handle_or_last(function, raw)?;
        Ok(Self {
            raw,
            _callback: callback,
        })
    }

    pub fn copy_value(&self, key: &str) -> Result<Option<PropertyList>> {
        let key = bridge::cstring(key, "sc_dynamic_store_copy_value")?;
        let raw = unsafe {
            ffi::dynamic_store::sc_dynamic_store_copy_value(self.raw.as_ptr(), key.as_ptr())
        };
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
            ffi::dynamic_store::sc_dynamic_store_add_value(
                self.raw.as_ptr(),
                key.as_ptr(),
                value.as_ptr(),
            )
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
            ffi::dynamic_store::sc_dynamic_store_set_value(
                self.raw.as_ptr(),
                key.as_ptr(),
                value.as_ptr(),
            )
        };
        bridge::bool_result("sc_dynamic_store_set_value", ok)
    }

    pub fn set_multiple<R, N>(
        &self,
        keys_to_set: Option<&PropertyList>,
        keys_to_remove: &[R],
        keys_to_notify: &[N],
    ) -> Result<()>
    where
        R: AsRef<str>,
        N: AsRef<str>,
    {
        let keys_to_remove = CStringArray::new(keys_to_remove, "sc_dynamic_store_set_multiple")?;
        let keys_to_notify = CStringArray::new(keys_to_notify, "sc_dynamic_store_set_multiple")?;
        let ok = unsafe {
            ffi::dynamic_store::sc_dynamic_store_set_multiple(
                self.raw.as_ptr(),
                keys_to_set.map_or(std::ptr::null_mut(), PropertyList::as_ptr),
                keys_to_remove.as_ptr(),
                keys_to_remove.count(),
                keys_to_notify.as_ptr(),
                keys_to_notify.count(),
            )
        };
        bridge::bool_result("sc_dynamic_store_set_multiple", ok)
    }

    pub fn remove_value(&self, key: &str) -> Result<()> {
        let key = bridge::cstring(key, "sc_dynamic_store_remove_value")?;
        let ok = unsafe {
            ffi::dynamic_store::sc_dynamic_store_remove_value(self.raw.as_ptr(), key.as_ptr())
        };
        bridge::bool_result("sc_dynamic_store_remove_value", ok)
    }

    pub fn notify_value(&self, key: &str) -> Result<()> {
        let key = bridge::cstring(key, "sc_dynamic_store_notify_value")?;
        let ok = unsafe {
            ffi::dynamic_store::sc_dynamic_store_notify_value(self.raw.as_ptr(), key.as_ptr())
        };
        bridge::bool_result("sc_dynamic_store_notify_value", ok)
    }

    pub fn copy_key_list(&self, pattern: &str) -> Result<Vec<String>> {
        let pattern = bridge::cstring(pattern, "sc_dynamic_store_copy_key_list")?;
        let raw = unsafe {
            ffi::dynamic_store::sc_dynamic_store_copy_key_list(self.raw.as_ptr(), pattern.as_ptr())
        };
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

    pub fn create_run_loop_source(&self, order: isize) -> Result<DynamicStoreRunLoopSource> {
        let raw = unsafe {
            ffi::dynamic_store::sc_dynamic_store_create_run_loop_source(self.raw.as_ptr(), order)
        };
        let raw = bridge::owned_handle_or_last("sc_dynamic_store_create_run_loop_source", raw)?;
        Ok(DynamicStoreRunLoopSource { raw })
    }

    pub fn set_dispatch_queue_global(&self) -> Result<()> {
        let ok = unsafe {
            ffi::dynamic_store::sc_dynamic_store_set_dispatch_queue_global(self.raw.as_ptr())
        };
        bridge::bool_result("sc_dynamic_store_set_dispatch_queue_global", ok)
    }

    pub fn clear_dispatch_queue(&self) -> Result<()> {
        let ok =
            unsafe { ffi::dynamic_store::sc_dynamic_store_clear_dispatch_queue(self.raw.as_ptr()) };
        bridge::bool_result("sc_dynamic_store_clear_dispatch_queue", ok)
    }

    pub fn copy_notified_keys(&self) -> Vec<String> {
        let raw =
            unsafe { ffi::dynamic_store::sc_dynamic_store_copy_notified_keys(self.raw.as_ptr()) };
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
                service_id
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    pub fn dhcp_option_data(info: &PropertyList, code: u8) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(ffi::dynamic_store::sc_dhcp_info_copy_option_data(
                info.as_ptr(),
                code,
            ))
        }
        .map(PropertyList::from_owned_handle)
    }

    pub fn dhcp_lease_start_time(info: &PropertyList) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(ffi::dynamic_store::sc_dhcp_info_copy_lease_start_time(
                info.as_ptr(),
            ))
        }
        .map(PropertyList::from_owned_handle)
    }

    pub fn dhcp_lease_expiration_time(info: &PropertyList) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::dynamic_store::sc_dhcp_info_copy_lease_expiration_time(info.as_ptr()),
            )
        }
        .map(PropertyList::from_owned_handle)
    }

    pub fn key_create<A: AsRef<str>>(format: &str, arguments: &[A]) -> Result<String> {
        let format = bridge::cstring(format, "sc_dynamic_store_key_create")?;
        let arguments = CStringArray::new(arguments, "sc_dynamic_store_key_create")?;
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create(
                format.as_ptr(),
                arguments.as_ptr(),
                arguments.count(),
            )
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create",
                "bridge returned null dynamic-store formatted key",
            )
        })
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
        let domain = bridge::cstring(
            domain,
            "sc_dynamic_store_key_create_network_interface_entity",
        )?;
        let interface_name = bridge::cstring(
            interface_name,
            "sc_dynamic_store_key_create_network_interface_entity",
        )?;
        let entity = bridge::optional_cstring(
            entity,
            "sc_dynamic_store_key_create_network_interface_entity",
        )?;
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_network_interface_entity(
                domain.as_ptr(),
                interface_name.as_ptr(),
                entity
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
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
        let service_id = bridge::cstring(
            service_id,
            "sc_dynamic_store_key_create_network_service_entity",
        )?;
        let entity =
            bridge::optional_cstring(entity, "sc_dynamic_store_key_create_network_service_entity")?;
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_network_service_entity(
                domain.as_ptr(),
                service_id.as_ptr(),
                entity
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
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
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_computer_name()
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create_computer_name",
                "bridge returned null computer-name notification key",
            )
        })
    }

    pub fn console_user_key() -> Result<String> {
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_console_user()
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create_console_user",
                "bridge returned null console-user notification key",
            )
        })
    }

    pub fn host_names_key() -> Result<String> {
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_host_names()
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create_host_names",
                "bridge returned null host-names notification key",
            )
        })
    }

    pub fn location_key() -> Result<String> {
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_location()
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create_location",
                "bridge returned null location notification key",
            )
        })
    }

    pub fn proxies_key() -> Result<String> {
        bridge::take_optional_string(unsafe {
            ffi::dynamic_store::sc_dynamic_store_key_create_proxies()
        })
        .ok_or_else(|| {
            SystemConfigurationError::null(
                "sc_dynamic_store_key_create_proxies",
                "bridge returned null proxies notification key",
            )
        })
    }
}

impl DynamicStoreRunLoopSource {
    pub fn schedule_current_default_mode(&self) -> Result<()> {
        let ok = unsafe {
            ffi::dynamic_store::sc_run_loop_source_schedule_current_default_mode(self.raw.as_ptr())
        };
        bridge::bool_result("sc_run_loop_source_schedule_current_default_mode", ok)
    }

    pub fn unschedule_current_default_mode(&self) -> Result<()> {
        let ok = unsafe {
            ffi::dynamic_store::sc_run_loop_source_unschedule_current_default_mode(
                self.raw.as_ptr(),
            )
        };
        bridge::bool_result("sc_run_loop_source_unschedule_current_default_mode", ok)
    }
}
