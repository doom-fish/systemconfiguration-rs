use std::{
    cell::RefCell,
    ffi::c_void,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
    ptr::NonNull,
    rc::Rc,
    sync::Mutex,
};

use apple_cf::{cf::CFRunLoop, dispatch_queue::DispatchQueue};
use doom_fish_utils::callback_context::CallbackContext;

use crate::{
    bridge, error::Result, ffi, network_services::NetworkService, PropertyList, RunLoopMode,
};

type PreferencesCallback = bridge::CallbackSlot<dyn FnMut(PreferencesNotification) + Send>;
type PreferencesCallbackContext = CallbackContext<PreferencesCallback>;

unsafe extern "C" fn preferences_callback(notification_type: u32, info: *mut c_void) {
    unsafe {
        PreferencesCallbackContext::with(info, "preferences_callback", |slot| {
            bridge::with_callback(slot, |callback| {
                callback(PreferencesNotification::from_raw(notification_type));
            });
        });
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

struct PreferencesInner {
    raw: bridge::OwnedHandle,
    callback: RefCell<Option<PreferencesCallbackContext>>,
}

impl Drop for PreferencesInner {
    fn drop(&mut self) {
        if let Some(callback) = self.callback.get_mut() {
            bridge::retire_callback(callback);
        }
    }
}

#[derive(Clone)]
/// Wraps `SCPreferencesRef`.
pub struct Preferences {
    inner: Rc<PreferencesInner>,
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
        let preferences = Self::new(name, prefs_id)?;
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
        let preferences = Self::new_with_authorization(name, prefs_id)?;
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
        let preferences =
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
        Ok(Self::from_owned_handle(raw))
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
        Ok(Self::from_owned_handle(raw))
    }

    fn from_owned_handle(raw: bridge::OwnedHandle) -> Self {
        Self {
            inner: Rc::new(PreferencesInner {
                raw,
                callback: RefCell::new(None),
            }),
        }
    }

    /// Wraps a helper on `SCPreferencesRef`.
    pub fn set_callback<F>(&self, callback: F) -> Result<()>
    where
        F: FnMut(PreferencesNotification) + Send + 'static,
    {
        let context = PreferencesCallbackContext::new(Mutex::new(Some(Box::new(callback))));
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_callback(
                self.as_ptr(),
                Some(preferences_callback),
                context.as_ptr(),
                Some(PreferencesCallbackContext::RETAIN),
                Some(PreferencesCallbackContext::RELEASE),
            )
        };
        bridge::bool_result("sc_preferences_set_callback", ok)?;
        let previous = self.inner.callback.replace(Some(context));
        if let Some(previous) = &previous {
            bridge::retire_callback(previous);
        }
        drop(previous);
        Ok(())
    }

    /// Wraps a helper on `SCPreferencesRef`.
    pub fn clear_callback(&self) -> Result<()> {
        if let Some(previous) = self.inner.callback.borrow().as_ref() {
            bridge::retire_callback(previous);
        }
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_callback(
                self.as_ptr(),
                None,
                std::ptr::null_mut(),
                None,
                None,
            )
        };
        bridge::bool_result("sc_preferences_set_callback", ok)?;
        drop(self.inner.callback.take());
        Ok(())
    }

    pub fn schedule_with_run_loop(
        &self,
        run_loop: &CFRunLoop,
        mode: RunLoopMode<'_>,
    ) -> Result<()> {
        let ok = mode.with_raw(|mode| unsafe {
            ffi::preferences::sc_preferences_schedule_with_run_loop(
                self.as_ptr(),
                run_loop.as_ptr(),
                mode,
            )
        });
        bridge::bool_result("sc_preferences_schedule_with_run_loop", ok)
    }

    pub fn unschedule_from_run_loop(
        &self,
        run_loop: &CFRunLoop,
        mode: RunLoopMode<'_>,
    ) -> Result<()> {
        let ok = mode.with_raw(|mode| unsafe {
            ffi::preferences::sc_preferences_unschedule_from_run_loop(
                self.as_ptr(),
                run_loop.as_ptr(),
                mode,
            )
        });
        bridge::bool_result("sc_preferences_unschedule_from_run_loop", ok)
    }

    /// Wraps `SCPreferencesScheduleWithRunLoopCurrent`.
    pub fn schedule_with_run_loop_current(&self) -> Result<()> {
        self.schedule_with_run_loop(&CFRunLoop::current(), RunLoopMode::Default)
    }

    /// Wraps `SCPreferencesUnscheduleFromRunLoopCurrent`.
    pub fn unschedule_from_run_loop_current(&self) -> Result<()> {
        self.unschedule_from_run_loop(&CFRunLoop::current(), RunLoopMode::Default)
    }

    pub fn set_dispatch_queue(&self, queue: &DispatchQueue) -> Result<()> {
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_dispatch_queue(
                self.as_ptr(),
                queue.as_ptr().cast_mut(),
            )
        };
        bridge::bool_result("sc_preferences_set_dispatch_queue", ok)
    }

    /// Wraps `SCPreferencesSetDispatchQueueGlobal`.
    pub fn set_dispatch_queue_global(&self) -> Result<()> {
        let ok =
            unsafe { ffi::preferences::sc_preferences_set_dispatch_queue_global(self.as_ptr()) };
        bridge::bool_result("sc_preferences_set_dispatch_queue_global", ok)
    }

    /// Wraps `SCPreferencesClearDispatchQueue`.
    pub fn clear_dispatch_queue(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_clear_dispatch_queue(self.as_ptr()) };
        bridge::bool_result("sc_preferences_clear_dispatch_queue", ok)
    }

    /// Wraps `SCPreferencesLock`.
    pub fn lock(&self, wait: bool) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_lock(self.as_ptr(), u8::from(wait)) };
        bridge::bool_result("sc_preferences_lock", ok)
    }

    /// Wraps `SCPreferencesCommitChanges`.
    pub fn commit_changes(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_commit_changes(self.as_ptr()) };
        bridge::bool_result("sc_preferences_commit_changes", ok)
    }

    /// Wraps `SCPreferencesApplyChanges`.
    pub fn apply_changes(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_apply_changes(self.as_ptr()) };
        bridge::bool_result("sc_preferences_apply_changes", ok)
    }

    /// Wraps `SCPreferencesUnlock`.
    pub fn unlock(&self) -> Result<()> {
        let ok = unsafe { ffi::preferences::sc_preferences_unlock(self.as_ptr()) };
        bridge::bool_result("sc_preferences_unlock", ok)
    }

    /// Wraps `SCPreferencesSynchronize`.
    pub fn synchronize(&self) {
        unsafe { ffi::preferences::sc_preferences_synchronize(self.as_ptr()) };
    }

    /// Wraps `SCPreferencesCopySignature`.
    pub fn signature(&self) -> Option<String> {
        bridge::take_optional_string(unsafe {
            ffi::preferences::sc_preferences_copy_signature(self.as_ptr())
        })
    }

    /// Wraps `SCPreferencesCopyKeyList`.
    pub fn copy_key_list(&self) -> Vec<String> {
        bridge::take_string_array(unsafe {
            ffi::preferences::sc_preferences_copy_key_list(self.as_ptr())
        })
    }

    /// Wraps `SCPreferencesGetValue`.
    pub fn get_value(&self, key: &str) -> Result<Option<PropertyList>> {
        let key = bridge::cstring(key, "sc_preferences_get_value")?;
        let raw =
            unsafe { ffi::preferences::sc_preferences_get_value(self.as_ptr(), key.as_ptr()) };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    /// Wraps `SCPreferencesAddValue`.
    pub fn add_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = bridge::cstring(key, "sc_preferences_add_value")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_add_value(self.as_ptr(), key.as_ptr(), value.as_ptr())
        };
        bridge::bool_result("sc_preferences_add_value", ok)
    }

    /// Wraps `SCPreferencesSetValue`.
    pub fn set_value(&self, key: &str, value: &PropertyList) -> Result<()> {
        let key = bridge::cstring(key, "sc_preferences_set_value")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_value(self.as_ptr(), key.as_ptr(), value.as_ptr())
        };
        bridge::bool_result("sc_preferences_set_value", ok)
    }

    /// Wraps `SCPreferencesRemoveValue`.
    pub fn remove_value(&self, key: &str) -> Result<()> {
        let key = bridge::cstring(key, "sc_preferences_remove_value")?;
        let ok =
            unsafe { ffi::preferences::sc_preferences_remove_value(self.as_ptr(), key.as_ptr()) };
        bridge::bool_result("sc_preferences_remove_value", ok)
    }

    /// Wraps `SCPreferencesPathCreateUniqueChild`.
    pub fn path_create_unique_child(&self, prefix: &str) -> Result<Option<String>> {
        let prefix = bridge::cstring(prefix, "sc_preferences_path_create_unique_child")?;
        Ok(bridge::take_optional_string(unsafe {
            ffi::preferences::sc_preferences_path_create_unique_child(
                self.as_ptr(),
                prefix.as_ptr(),
            )
        }))
    }

    /// Wraps `SCPreferencesPathGetValue`.
    pub fn path_get_value(&self, path: &str) -> Result<Option<PropertyList>> {
        let path = bridge::cstring(path, "sc_preferences_path_get_value")?;
        let raw = unsafe {
            ffi::preferences::sc_preferences_path_get_value(self.as_ptr(), path.as_ptr())
        };
        Ok(unsafe { bridge::OwnedHandle::from_raw(raw) }.map(PropertyList::from_owned_handle))
    }

    /// Wraps `SCPreferencesPathGetLink`.
    pub fn path_get_link(&self, path: &str) -> Result<Option<String>> {
        let path = bridge::cstring(path, "sc_preferences_path_get_link")?;
        Ok(bridge::take_optional_string(unsafe {
            ffi::preferences::sc_preferences_path_get_link(self.as_ptr(), path.as_ptr())
        }))
    }

    /// Wraps `SCPreferencesPathSetValue`.
    pub fn path_set_value(&self, path: &str, value: &PropertyList) -> Result<()> {
        let path = bridge::cstring(path, "sc_preferences_path_set_value")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_path_set_value(
                self.as_ptr(),
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
                self.as_ptr(),
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
            ffi::preferences::sc_preferences_path_remove_value(self.as_ptr(), path.as_ptr())
        };
        bridge::bool_result("sc_preferences_path_remove_value", ok)
    }

    /// Wraps `SCPreferencesSetComputerName`.
    pub fn set_computer_name(&self, name: Option<&str>) -> Result<()> {
        let name = bridge::optional_cstring(name, "sc_preferences_set_computer_name")?;
        let ok = unsafe {
            ffi::preferences::sc_preferences_set_computer_name(
                self.as_ptr(),
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
                self.as_ptr(),
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
        self.inner.raw.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        path::PathBuf,
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    };

    use apple_cf::{
        cf::CFRunLoop,
        dispatch_queue::{DispatchQoS, DispatchQueue},
    };

    use super::Preferences;
    use crate::{
        bridge::test_support::{run_loop_mode_is_empty, wait_for, InFlightCall},
        RunLoopMode,
    };

    fn temporary_preferences(name: &str) -> Preferences {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/test-data");
        std::fs::create_dir_all(&dir).expect("test data dir");
        let path = dir.join(format!("{name}-{nanos}.plist"));
        Preferences::new(
            "systemconfiguration-rs-unit-tests",
            Some(path.to_string_lossy().as_ref()),
        )
        .expect("preferences")
    }

    fn set_witness_callback(prefs: &Preferences) -> Arc<()> {
        let witness = Arc::new(());
        let captured = Arc::clone(&witness);
        prefs
            .set_callback(move |_| {
                let _ = &captured;
            })
            .expect("callback");
        witness
    }

    fn hold(prefs: &Preferences) -> InFlightCall {
        InFlightCall::hold(prefs.inner.callback.borrow().as_ref().expect("callback"))
    }

    #[test]
    fn dropping_scheduled_preferences_releases_the_callback_after_an_in_flight_call() {
        let prefs = temporary_preferences("unit-run-loop");
        let witness = set_witness_callback(&prefs);
        let run_loop = CFRunLoop::current();
        prefs
            .schedule_with_run_loop(&run_loop, RunLoopMode::Default)
            .expect("schedule default");
        let mode = "systemconfiguration-rs.unit-prefs-mode";
        prefs
            .schedule_with_run_loop(&run_loop, RunLoopMode::Named(mode))
            .expect("schedule named");
        assert!(!run_loop_mode_is_empty(mode));

        let call = hold(&prefs);
        drop(prefs);
        assert!(run_loop_mode_is_empty(mode));
        assert_eq!(Arc::strong_count(&witness), 2);

        call.finish();
        assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    }

    #[test]
    fn replacing_a_callback_during_an_in_flight_call_keeps_the_old_one_alive() {
        let prefs = temporary_preferences("unit-replace");
        prefs
            .set_dispatch_queue(&DispatchQueue::new(
                "systemconfiguration-rs.unit-prefs",
                DispatchQoS::Utility,
            ))
            .expect("dispatch queue");
        let first = set_witness_callback(&prefs);

        let call = hold(&prefs);
        let second = set_witness_callback(&prefs);
        assert_eq!(Arc::strong_count(&first), 2);

        call.finish();
        assert!(wait_for(|| Arc::strong_count(&first) == 1));
        assert_eq!(Arc::strong_count(&second), 2);

        let call = hold(&prefs);
        drop(prefs);
        call.finish();
        assert!(wait_for(|| Arc::strong_count(&second) == 1));
    }
}
