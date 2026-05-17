#![cfg(feature = "async")]

//! Async event streams for `SystemConfiguration` callbacks.
//!
//! Enable this module with `--features async`.

use std::ffi::c_void;

use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::{
    bridge::{self, CStringArray},
    error::Result,
    ffi, PreferencesNotification, ReachabilityFlags, SystemConfigurationError,
};

struct SubscriptionHandle {
    ptr: *mut c_void,
    sender_ptr: *mut c_void,
    unsubscribe_fn: unsafe extern "C" fn(*mut c_void),
    drop_sender_fn: unsafe fn(*mut c_void),
}

impl SubscriptionHandle {
    fn new<T>(
        ptr: *mut c_void,
        sender_ptr: *mut AsyncStreamSender<T>,
        unsubscribe_fn: unsafe extern "C" fn(*mut c_void),
    ) -> Self {
        Self {
            ptr,
            sender_ptr: sender_ptr.cast::<c_void>(),
            unsubscribe_fn,
            drop_sender_fn: drop_sender::<T>,
        }
    }
}

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { (self.unsubscribe_fn)(self.ptr) };
        }
        if !self.sender_ptr.is_null() {
            unsafe { (self.drop_sender_fn)(self.sender_ptr) };
        }
    }
}

unsafe impl Send for SubscriptionHandle {}
unsafe impl Sync for SubscriptionHandle {}

unsafe fn drop_sender<T>(raw: *mut c_void) {
    if raw.is_null() {
        return;
    }
    unsafe { drop(Box::from_raw(raw.cast::<AsyncStreamSender<T>>())); }
}

#[derive(Debug, Clone)]
pub struct DynamicStoreNotificationEvent {
    pub changed_keys: Vec<String>,
}

pub struct DynamicStoreNotificationStream {
    inner: BoundedAsyncStream<DynamicStoreNotificationEvent>,
    _handle: SubscriptionHandle,
}

unsafe extern "C" fn dynamic_store_async_cb(kind: i32, payload: *mut c_void, ctx: *mut c_void) {
    if kind != 0 || ctx.is_null() {
        return;
    }

    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<DynamicStoreNotificationEvent>>() };
    let changed_keys = if payload.is_null() {
        Vec::new()
    } else {
        bridge::take_string_array(payload)
    };
    sender.push(DynamicStoreNotificationEvent { changed_keys });
}

impl DynamicStoreNotificationStream {
    pub fn subscribe(name: &str, keys: &[&str], patterns: &[&str], capacity: usize) -> Result<Self> {
        let c_name = bridge::cstring(name, "sc_dynamic_store_notification_subscribe")?;
        let c_keys = CStringArray::new(keys, "sc_dynamic_store_notification_subscribe")?;
        let c_patterns = CStringArray::new(patterns, "sc_dynamic_store_notification_subscribe")?;

        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));

        let handle_ptr = unsafe {
            ffi::async_api::sc_dynamic_store_notification_subscribe(
                c_name.as_ptr(),
                c_keys.as_ptr(),
                c_keys.count(),
                c_patterns.as_ptr(),
                c_patterns.count(),
                Some(dynamic_store_async_cb),
                sender_ptr.cast::<c_void>(),
            )
        };

        if handle_ptr.is_null() {
            unsafe { drop(Box::from_raw(sender_ptr)); }
            return Err(SystemConfigurationError::last(
                "sc_dynamic_store_notification_subscribe",
            ));
        }

        Ok(Self {
            inner: stream,
            _handle: SubscriptionHandle::new(
                handle_ptr,
                sender_ptr,
                ffi::async_api::sc_dynamic_store_notification_unsubscribe,
            ),
        })
    }

    pub const fn next(&self) -> NextItem<'_, DynamicStoreNotificationEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<DynamicStoreNotificationEvent> {
        self.inner.try_next()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReachabilityEvent {
    pub flags: ReachabilityFlags,
}

pub struct ReachabilityStream {
    inner: BoundedAsyncStream<ReachabilityEvent>,
    _handle: SubscriptionHandle,
}

unsafe extern "C" fn reachability_async_cb(kind: i32, _payload: *mut c_void, ctx: *mut c_void) {
    if ctx.is_null() {
        return;
    }

    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<ReachabilityEvent>>() };
    sender.push(ReachabilityEvent {
        flags: ReachabilityFlags(u32::from_ne_bytes(kind.to_ne_bytes())),
    });
}

impl ReachabilityStream {
    pub fn subscribe(name: &str, capacity: usize) -> Result<Self> {
        let c_name = bridge::cstring(name, "sc_reachability_notification_subscribe")?;

        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));

        let handle_ptr = unsafe {
            ffi::async_api::sc_reachability_notification_subscribe(
                c_name.as_ptr(),
                Some(reachability_async_cb),
                sender_ptr.cast::<c_void>(),
            )
        };

        if handle_ptr.is_null() {
            unsafe { drop(Box::from_raw(sender_ptr)); }
            return Err(SystemConfigurationError::last(
                "sc_reachability_notification_subscribe",
            ));
        }

        Ok(Self {
            inner: stream,
            _handle: SubscriptionHandle::new(
                handle_ptr,
                sender_ptr,
                ffi::async_api::sc_reachability_notification_unsubscribe,
            ),
        })
    }

    pub const fn next(&self) -> NextItem<'_, ReachabilityEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<ReachabilityEvent> {
        self.inner.try_next()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PreferencesNotificationEvent {
    pub notification: PreferencesNotification,
}

pub struct PreferencesNotificationStream {
    inner: BoundedAsyncStream<PreferencesNotificationEvent>,
    _handle: SubscriptionHandle,
}

unsafe extern "C" fn preferences_async_cb(kind: i32, _payload: *mut c_void, ctx: *mut c_void) {
    if ctx.is_null() {
        return;
    }

    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<PreferencesNotificationEvent>>() };
    sender.push(PreferencesNotificationEvent {
        notification: PreferencesNotification::from_raw(u32::from_ne_bytes(kind.to_ne_bytes())),
    });
}

impl PreferencesNotificationStream {
    pub fn subscribe(name: &str, capacity: usize) -> Result<Self> {
        let c_name = bridge::cstring(name, "sc_preferences_notification_subscribe")?;

        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender));

        let handle_ptr = unsafe {
            ffi::async_api::sc_preferences_notification_subscribe(
                c_name.as_ptr(),
                Some(preferences_async_cb),
                sender_ptr.cast::<c_void>(),
            )
        };

        if handle_ptr.is_null() {
            unsafe { drop(Box::from_raw(sender_ptr)); }
            return Err(SystemConfigurationError::last(
                "sc_preferences_notification_subscribe",
            ));
        }

        Ok(Self {
            inner: stream,
            _handle: SubscriptionHandle::new(
                handle_ptr,
                sender_ptr,
                ffi::async_api::sc_preferences_notification_unsubscribe,
            ),
        })
    }

    pub const fn next(&self) -> NextItem<'_, PreferencesNotificationEvent> {
        self.inner.next()
    }

    pub fn try_next(&self) -> Option<PreferencesNotificationEvent> {
        self.inner.try_next()
    }

    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
}
