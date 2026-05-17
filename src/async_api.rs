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

// SAFETY: `SubscriptionHandle` owns two raw pointers that are each used
// exactly once (in `Drop`).  The `ptr` was returned by a Swift subscribe
// thunk and is only ever passed back into the matching unsubscribe thunk.
// The `sender_ptr` was created with `Box::into_raw` and is reconstituted
// exactly once via `drop_sender_fn`.  Neither pointer is shared or aliased
// elsewhere, so transferring the handle across thread boundaries is safe.
unsafe impl Send for SubscriptionHandle {}
// SAFETY: `SubscriptionHandle` carries no shared mutable state — the two
// pointers are used only in `Drop` (single-caller, single-use).  It is safe
// to hold a `&SubscriptionHandle` on multiple threads simultaneously because
// no method takes `&self`.
unsafe impl Sync for SubscriptionHandle {}

unsafe fn drop_sender<T>(raw: *mut c_void) {
    if raw.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(raw.cast::<AsyncStreamSender<T>>()));
    }
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

    // SAFETY: `ctx` is the `sender_ptr` created in `subscribe` via
    // `Box::into_raw`.  It remains valid for the lifetime of the
    // `SubscriptionHandle` (which outlives every callback invocation because
    // `unsubscribe` joins the run-loop thread before `Drop` frees the sender).
    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<DynamicStoreNotificationEvent>>() };
    let changed_keys = if payload.is_null() {
        Vec::new()
    } else {
        bridge::take_string_array(payload)
    };
    doom_fish_utils::panic_safe::catch_user_panic(
        "dynamic_store_async_cb",
        || sender.push(DynamicStoreNotificationEvent { changed_keys }),
    );
}

impl DynamicStoreNotificationStream {
    pub fn subscribe(
        name: &str,
        keys: &[&str],
        patterns: &[&str],
        capacity: usize,
    ) -> Result<Self> {
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
            unsafe {
                drop(Box::from_raw(sender_ptr));
            }
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

    // SAFETY: same lifetime contract as `dynamic_store_async_cb` — `ctx` is a
    // `Box::into_raw`-leaked `AsyncStreamSender<ReachabilityEvent>` that lives
    // until `SubscriptionHandle::Drop` calls `drop_sender_fn`.  Reachability
    // uses a dispatch queue (not a dedicated thread), so the queue is stopped
    // before the sender is freed.
    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<ReachabilityEvent>>() };
    doom_fish_utils::panic_safe::catch_user_panic("reachability_async_cb", || {
        sender.push(ReachabilityEvent {
            flags: ReachabilityFlags(u32::from_ne_bytes(kind.to_ne_bytes())),
        });
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
            unsafe {
                drop(Box::from_raw(sender_ptr));
            }
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

    // SAFETY: same lifetime contract as `dynamic_store_async_cb` — `ctx` is a
    // `Box::into_raw`-leaked `AsyncStreamSender<PreferencesNotificationEvent>`
    // that lives until `SubscriptionHandle::Drop` calls `drop_sender_fn`.
    let sender = unsafe { &*ctx.cast::<AsyncStreamSender<PreferencesNotificationEvent>>() };
    doom_fish_utils::panic_safe::catch_user_panic("preferences_async_cb", || {
        sender.push(PreferencesNotificationEvent {
            notification: PreferencesNotification::from_raw(u32::from_ne_bytes(
                kind.to_ne_bytes(),
            )),
        });
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
            unsafe {
                drop(Box::from_raw(sender_ptr));
            }
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
