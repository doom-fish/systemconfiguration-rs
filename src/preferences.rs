use std::{
    ffi::c_void,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
    ptr::NonNull,
    sync::{Arc, Mutex},
};

use crate::{bridge, error::Result, ffi, network_services::NetworkService, PropertyList};

struct CallbackState {
    callback: Box<dyn FnMut(PreferencesNotification) + Send>,
}

unsafe extern "C" fn preferences_callback(notification_type: u32, info: *mut c_void) {
    if info.is_null() {
        return;
    }

    let mutex = unsafe { &*info.cast::<Mutex<CallbackState>>() };
    if let Ok(mut state) = mutex.lock() {
        (state.callback)(PreferencesNotification::from_raw(notification_type));
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Wraps `SCPreferencesNotification`.
pub struct PreferencesNotification(u32);

impl PreferencesNotification {
    /// Wraps `kSCPreferencesNotificationCommit`.
    pub const COMMIT: Self = Self(1 << 0);
    /// Wraps `kSCPreferencesNotificationApply`.
    pub const APPLY: Self = Self(1 << 1);

    /// Wraps conversion from raw `SCPreferencesNotification` values.
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Wraps conversion to raw `SCPreferencesNotification` values.
    pub const fn raw_value(self) -> u32 {
        self.0
    }

    /// Wraps membership checks on `SCPreferencesNotification` flags.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl BitOr for PreferencesNotification {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for PreferencesNotification {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for PreferencesNotification {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for PreferencesNotification {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

#[derive(Clone)]
/// Wraps `SCPreferencesRef`.
pub struct Preferences {
    raw: bridge::OwnedHandle,
    callback_state: Option<Arc<Mutex<CallbackState>>>,
}

impl std::fmt::Debug for Preferences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Preferences").finish_non_exhaustive()
    }
}

impl Preferences {
    /// Wraps `SCPreferencesGetTypeID`.
    pub fn type_id() -> u64 {
        unsafe { ffi::preferences::sc_preferences_get_type_id() }
    }

    /// Wraps `SCPreferencesCreate`.
    pub fn new(name: &str, prefs_id: Option<&str>) -> Result<Self> {
        Self::create(name, prefs_id)
    }

    /// Wraps `SCPreferencesCreateWithAuthorization`.
    pub fn new_with_authorization(name: &str, prefs_id: Option<&str>) -> Result<Self> {
        unsafe { Self::create_with_authorization(name, prefs_id, None) }
    }

    /// # Safety
    ///
    /// `authorization` must be a valid `AuthorizationRef` obtained from a
    /// compatible Security-framework binding, and it must remain valid for the
    /// lifetime requirements imposed by `SCPreferences`.
    pub unsafe fn new_with_authorization_raw(
        name: &str,
        prefs_id: Option<&str>,
        authorization: Option<NonNull<c_void>>,
    ) -> Result<Self> {
        unsafe { Self::create_with_authorization(name, prefs_id, authorization) }
    }

    /// Wraps `SCPreferencesCreate` with `SCPreferencesSetCallback`.
    pub fn new_with_callback<F>(name: &str, prefs_id: Option<&str>, callback: F) -> Result<Self>
    where
        F: FnMut(PreferencesNotification) + Send + 'static,
    {
        let mut preferences = Self::new(name, prefs_id)?;
        preferences.set_callback(callback)?;
        Ok(preferences)
    }

    /// Wraps `SCPreferencesCreateWithAuthorization` with `SCPreferencesSetCallback`.
    pub fn new_with_authorization_and_callback<F>(
        name: &str,
        prefs_id: Option<&str>,
        callback: F,
    ) -> Result<Self>
    where
        F: FnMut(PreferencesNotification) + Send + 'static,
    {
        let mut preferences = Self::new_with_authorization(name, prefs_id)?;
        preferences.set_callback(callback)?;
        Ok(preferences)
    }

    /// # Safety
    ///
    /// `authorization` must be a valid `AuthorizationRef` obtained from a
    /// compatible Security-framework binding, and it must remain valid for the
    /// lifetime requirements imposed by `SCPreferences`.
    pub unsafe fn new_with_authorization_raw_and_callback<F>(
        name: &str,
        prefs_id: Option<&str>,
        authorization: Option<NonNull<c_void>>,
        callback: F,
    ) -> Result<Self>
    where
        F: FnMut(PreferencesNotification) + Send + 'static,
    {
        let mut preferences =
            unsafe { Self::create_with_authorization(name, prefs_id, authorization) }?;
        preferences.set_callback(callback)?;
        Ok(preferences)
    }

    fn create(name: &str, prefs_id: Option<&str>) -> Result<Self> {
        let name = bridge::cstring(name, "sc_preferences_create")?;
        let prefs_id = bridge::optional_cstring(prefs_id, "sc_preferences_create")?;
        let raw = unsafe {
            ffi::preferences::sc_preferences_create(
                name.as_ptr(),
                prefs_id
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        let raw = bridge::owned_handle_or_last("sc_preferences_create", raw)?;
        Ok(Self {
            raw,
            callback_state: None,
        })
    }

    unsafe fn create_with_authorization(
        name: &str,
        prefs_id: Option<&str>,
        authorization: Option<NonNull<c_void>>,
    ) -> Result<Self> {
        let name = bridge::cstring(name, "sc_preferences_create_with_authorization")?;
        let prefs_id =
            bridge::optional_cstring(prefs_id, "sc_preferences_create_with_authorization")?;
        let raw = unsafe {
            ffi::preferences::sc_preferences_create_with_authorization(
                name.as_ptr(),
                prefs_id
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                authorization.map_or(std::ptr::null_mut(), NonNull::as_ptr),
            )
        };
        let raw = bridge::owned_handle_or_last("sc_preferences_create_with_authorization", raw)?;
        Ok(Self {
            raw,
            callback_state: None,
        })
    }

    /// Wraps a helper on `SCPreferencesRef`.
    pub fn set_callback<F>(&mut self, callback: F) -> Result<()>
    where
        F: FnMut(PreferencesNotification) + Send + 'static,
    {
        let state = Arc::new(Mutex::new(CallbackState {
            callback: Box::new(callback),
        }));
        self.set_callback_state(Some(state))
    }

    /// Wraps a helper on `SCPreferencesRef`.
    pub fn clear_callback(&mut self) -> Result<()> {
        self.set_callback_state(None)
    }

    fn set_callback_state(&mut self, callback: Option<Arc<Mutex<CallbackState>>>) -> Result<()> {
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_callback(
                self.raw.as_ptr(),
                callback
                    .as_ref()
                    .map(|_| preferences_callback as unsafe extern "C" fn(u32, *mut c_void)),
                callback.as_ref().map_or(std::ptr::null_mut(), |state| {
                    Arc::as_ptr(state).cast_mut().cast::<c_void>()
                }),
            )
        };
        bridge::bool_result("sc_preferences_set_callback", ok)?;
        self.callback_state = callback;
        Ok(())
    }

    /// Wraps `SCPreferencesScheduleWithRunLoopCurrent`.
    pub fn schedule_with_run_loop_current(&self) -> Result<()> {
        let ok = unsafe {
            ffi::preferences::sc_preferences_schedule_with_run_loop_current(self.raw.as_ptr())
        };
        bridge::bool_result("sc_preferences_schedule_with_run_loop_current", ok)
    }

    /// Wraps `SCPreferencesUnscheduleFromRunLoopCurrent`.
    pub fn unschedule_from_run_loop_current(&self) -> Result<()> {
        let ok = unsafe {
            ffi::preferences::sc_preferences_unschedule_from_run_loop_current(self.raw.as_ptr())
        };
        bridge::bool_result("sc_preferences_unschedule_from_run_loop_current", ok)
    }

    /// Wraps `SCPreferencesSetDispatchQueueGlobal`.
    pub fn set_dispatch_queue_global(&self) -> Result<()> {
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_dispatch_queue_global(self.raw.as_ptr())
        };
        bridge::bool_result("sc_preferences_set_dispatch_queue_global", ok)
    }

    /// Wraps `SCPreferencesClearDispatchQueue`.
    pub fn clear_dispatch_queue(&self) -> Result<()> {
        let ok =
            unsafe { ffi::preferences::sc_preferences_clear_dispatch_queue(self.raw.as_ptr()) };
        bridge::bool_result("sc_preferences_clear_dispatch_queue", ok)
    }

    /// Wraps `SCPreferencesLock`.
    pub fn lock(&self, wait: bool) -> Result<()> {
        let ok =
            unsafe { ffi::preferences::sc_preferences_lock(self.raw.as_ptr(), u8::from(wait)) };
        bridge::bool_result("sc_preferences_lock", ok)
    }

    /// Wraps `SCPreferencesCommitChanges`.
    pub fn commit_changes(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_commit_changes(self.raw.as_ptr()) };
        bridge::bool_result("sc_preferences_commit_changes", ok)
    }

    /// Wraps `SCPreferencesApplyChanges`.
    pub fn apply_changes(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_apply_changes(self.raw.as_ptr()) };
        bridge::bool_result("sc_preferences_apply_changes", ok)
    }

    /// Wraps `SCPreferencesUnlock`.
    pub fn unlock(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_unlock(self.raw.as_ptr()) };
        bridge::bool_result("sc_preferences_unlock", ok)
    }

    /// Wraps `SCPreferencesSynchronize`.
    pub fn synchronize(&self) {
        unsafe { ffi::preferences::sc_preferences_synchronize(self.raw.as_ptr()) };
    }

    /// Wraps `SCPreferencesCopySignature`.
    pub fn signature(&self) -> Option<String> {
        bridge::take_optional_string(unsafe {
            ffi::preferences::sc_preferences_copy_signature(self.raw.as_ptr())
        })
    }

    /// Wraps `SCPreferencesCopyKeyList`.
    pub fn copy_key_list(&self) -> Vec<String> {
        bridge::take_string_array(unsafe {
            ffi::preferences::sc_preferences_copy_key_list(self.raw.as_ptr())
        })
    }

    /// Wraps `SCPreferencesGetValue`.
    pub fn get_value(&self, key: &str) -> Result<Option<PropertyList>> {
        let key = bridge::cstring(key, "sc_preferences_get_value")?;
        let raw =
            unsafe { ffi::preferences::sc_preferences_get_value(self.raw.as_ptr(), key.as_ptr()) };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    /// Wraps `SCPreferencesAddValue`.
    pub fn add_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = bridge::cstring(key, "sc_preferences_add_value")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_add_value(
                self.raw.as_ptr(),
                key.as_ptr(),
                value.as_ptr(),
            )
        };
        bridge::bool_result("sc_preferences_add_value", ok)
    }

    /// Wraps `SCPreferencesSetValue`.
    pub fn set_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = bridge::cstring(key, "sc_preferences_set_value")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_value(
                self.raw.as_ptr(),
                key.as_ptr(),
                value.as_ptr(),
            )
        };
        bridge::bool_result("sc_preferences_set_value", ok)
    }

    /// Wraps `SCPreferencesRemoveValue`.
    pub fn remove_value(&self, key: &str) -> Result<()> {
        let key = bridge::cstring(key, "sc_preferences_remove_value")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_remove_value(self.raw.as_ptr(), key.as_ptr())
        };
        bridge::bool_result("sc_preferences_remove_value", ok)
    }

    /// Wraps `SCPreferencesPathCreateUniqueChild`.
    pub fn path_create_unique_child(&self, prefix: &str) -> Result<Option<String>> {
        let prefix = bridge::cstring(prefix, "sc_preferences_path_create_unique_child")?;
        Ok(bridge::take_optional_string(unsafe {
            ffi::preferences::sc_preferences_path_create_unique_child(
                self.raw.as_ptr(),
                prefix.as_ptr(),
            )
        }))
    }

    /// Wraps `SCPreferencesPathGetValue`.
    pub fn path_get_value(&self, path: &str) -> Result<Option<PropertyList>> {
        let path = bridge::cstring(path, "sc_preferences_path_get_value")?;
        let raw = unsafe {
            ffi::preferences::sc_preferences_path_get_value(self.raw.as_ptr(), path.as_ptr())
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    /// Wraps `SCPreferencesPathGetLink`.
    pub fn path_get_link(&self, path: &str) -> Result<Option<String>> {
        let path = bridge::cstring(path, "sc_preferences_path_get_link")?;
        Ok(bridge::take_optional_string(unsafe {
            ffi::preferences::sc_preferences_path_get_link(self.raw.as_ptr(), path.as_ptr())
        }))
    }

    /// Wraps `SCPreferencesPathSetValue`.
    pub fn path_set_value(&self, path: &str, value: &PropertyList) -> Result<()> {
        let path = bridge::cstring(path, "sc_preferences_path_set_value")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_path_set_value(
                self.raw.as_ptr(),
                path.as_ptr(),
                value.as_ptr(),
            )
        };
        bridge::bool_result("sc_preferences_path_set_value", ok)
    }

    /// Wraps `SCPreferencesPathSetLink`.
    pub fn path_set_link(&self, path: &str, link: &str) -> Result<()> {
        let path = bridge::cstring(path, "sc_preferences_path_set_link")?;
        let link = bridge::cstring(link, "sc_preferences_path_set_link")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_path_set_link(
                self.raw.as_ptr(),
                path.as_ptr(),
                link.as_ptr(),
            )
        };
        bridge::bool_result("sc_preferences_path_set_link", ok)
    }

    /// Wraps `SCPreferencesPathRemoveValue`.
    pub fn path_remove_value(&self, path: &str) -> Result<()> {
        let path = bridge::cstring(path, "sc_preferences_path_remove_value")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_path_remove_value(self.raw.as_ptr(), path.as_ptr())
        };
        bridge::bool_result("sc_preferences_path_remove_value", ok)
    }

    /// Wraps `SCPreferencesSetComputerName`.
    pub fn set_computer_name(&self, name: Option<&str>) -> Result<()> {
        let name = bridge::optional_cstring(name, "sc_preferences_set_computer_name")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_computer_name(
                self.raw.as_ptr(),
                name.as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        bridge::bool_result("sc_preferences_set_computer_name", ok)
    }

    /// Wraps `SCPreferencesSetLocalHostName`.
    pub fn set_local_host_name(&self, name: Option<&str>) -> Result<()> {
        let name = bridge::optional_cstring(name, "sc_preferences_set_local_host_name")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_local_host_name(
                self.raw.as_ptr(),
                name.as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
            )
        };
        bridge::bool_result("sc_preferences_set_local_host_name", ok)
    }

    /// Wraps a helper on `SCPreferencesRef`.
    pub fn network_services(&self) -> Vec<NetworkService> {
        NetworkService::copy_all(self)
    }

    pub(crate) fn as_ptr(&self) -> bridge::RawHandle {
        self.raw.as_ptr()
    }
}
