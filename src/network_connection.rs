use std::{
    ffi::c_void,
    sync::{Arc, Mutex},
};

use crate::{bridge, error::Result, ffi, PropertyList, ReachabilityFlags};

/// Alias for `SCNetworkConnectionFlags` values.
pub type NetworkConnectionFlags = ReachabilityFlags;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Wraps `SCNetworkConnectionStatus`.
pub enum NetworkConnectionStatus {
    /// Wraps the `Invalid` `SCNetworkConnectionStatus` value.
    Invalid,
    /// Wraps the `Disconnected` `SCNetworkConnectionStatus` value.
    Disconnected,
    /// Wraps the `Connecting` `SCNetworkConnectionStatus` value.
    Connecting,
    /// Wraps the `Connected` `SCNetworkConnectionStatus` value.
    Connected,
    /// Wraps the `Disconnecting` `SCNetworkConnectionStatus` value.
    Disconnecting,
    /// Wraps an unknown `SCNetworkConnectionStatus` value.
    Unknown(i32),
}

impl NetworkConnectionStatus {
    /// Wraps conversion from raw `SCNetworkConnectionStatus` values.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            -1 => Self::Invalid,
            0 => Self::Disconnected,
            1 => Self::Connecting,
            2 => Self::Connected,
            3 => Self::Disconnecting,
            other => Self::Unknown(other),
        }
    }

    /// Wraps conversion to raw `SCNetworkConnectionStatus` values.
    pub const fn raw_value(self) -> i32 {
        match self {
            Self::Invalid => -1,
            Self::Disconnected => 0,
            Self::Connecting => 1,
            Self::Connected => 2,
            Self::Disconnecting => 3,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Wraps `SCNetworkConnectionPPPStatus`.
pub enum NetworkConnectionPppStatus {
    /// Wraps the `Disconnected` `SCNetworkConnectionPPPStatus` value.
    Disconnected,
    /// Wraps the `Initializing` `SCNetworkConnectionPPPStatus` value.
    Initializing,
    /// Wraps the `ConnectingLink` `SCNetworkConnectionPPPStatus` value.
    ConnectingLink,
    /// Wraps the `DialOnTraffic` `SCNetworkConnectionPPPStatus` value.
    DialOnTraffic,
    /// Wraps the `NegotiatingLink` `SCNetworkConnectionPPPStatus` value.
    NegotiatingLink,
    /// Wraps the `Authenticating` `SCNetworkConnectionPPPStatus` value.
    Authenticating,
    /// Wraps the `WaitingForCallback` `SCNetworkConnectionPPPStatus` value.
    WaitingForCallback,
    /// Wraps the `NegotiatingNetwork` `SCNetworkConnectionPPPStatus` value.
    NegotiatingNetwork,
    /// Wraps the `Connected` `SCNetworkConnectionPPPStatus` value.
    Connected,
    /// Wraps the `Terminating` `SCNetworkConnectionPPPStatus` value.
    Terminating,
    /// Wraps the `DisconnectingLink` `SCNetworkConnectionPPPStatus` value.
    DisconnectingLink,
    /// Wraps the `HoldingLinkOff` `SCNetworkConnectionPPPStatus` value.
    HoldingLinkOff,
    /// Wraps the `Suspended` `SCNetworkConnectionPPPStatus` value.
    Suspended,
    /// Wraps the `WaitingForRedial` `SCNetworkConnectionPPPStatus` value.
    WaitingForRedial,
    /// Wraps an unknown `SCNetworkConnectionPPPStatus` value.
    Unknown(i32),
}

impl NetworkConnectionPppStatus {
    /// Wraps conversion from raw `SCNetworkConnectionPPPStatus` values.
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Disconnected,
            1 => Self::Initializing,
            2 => Self::ConnectingLink,
            3 => Self::DialOnTraffic,
            4 => Self::NegotiatingLink,
            5 => Self::Authenticating,
            6 => Self::WaitingForCallback,
            7 => Self::NegotiatingNetwork,
            8 => Self::Connected,
            9 => Self::Terminating,
            10 => Self::DisconnectingLink,
            11 => Self::HoldingLinkOff,
            12 => Self::Suspended,
            13 => Self::WaitingForRedial,
            other => Self::Unknown(other),
        }
    }

    /// Wraps conversion to raw `SCNetworkConnectionPPPStatus` values.
    pub const fn raw_value(self) -> i32 {
        match self {
            Self::Disconnected => 0,
            Self::Initializing => 1,
            Self::ConnectingLink => 2,
            Self::DialOnTraffic => 3,
            Self::NegotiatingLink => 4,
            Self::Authenticating => 5,
            Self::WaitingForCallback => 6,
            Self::NegotiatingNetwork => 7,
            Self::Connected => 8,
            Self::Terminating => 9,
            Self::DisconnectingLink => 10,
            Self::HoldingLinkOff => 11,
            Self::Suspended => 12,
            Self::WaitingForRedial => 13,
            Self::Unknown(raw) => raw,
        }
    }
}

#[derive(Clone, Debug)]
/// Wraps `SCNetworkConnectionCopyUserPreferences` results.
pub struct NetworkConnectionUserPreferences {
    /// Wraps the service identifier returned by `SCNetworkConnectionCopyUserPreferences`.
    pub service_id: String,
    /// Wraps the user options returned by `SCNetworkConnectionCopyUserPreferences`.
    pub user_options: Option<PropertyList>,
}

struct CallbackState {
    callback: Box<dyn FnMut(NetworkConnectionStatus) + Send>,
}

unsafe extern "C" fn network_connection_callback(status: i32, info: *mut c_void) {
    if info.is_null() {
        return;
    }

    let mutex = &*info.cast::<Mutex<CallbackState>>();
    if let Ok(mut state) = mutex.lock() {
        (state.callback)(NetworkConnectionStatus::from_raw(status));
    }
}

#[derive(Clone)]
/// Wraps `SCNetworkConnectionRef`.
pub struct NetworkConnection {
    raw: bridge::OwnedHandle,
    _callback: Option<Arc<Mutex<CallbackState>>>,
}

impl std::fmt::Debug for NetworkConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NetworkConnection").finish_non_exhaustive()
    }
}

impl NetworkConnection {
    /// Wraps `SCNetworkConnectionGetTypeID`.
    pub fn type_id() -> u64 {
        unsafe { ffi::network_connection::sc_network_connection_get_type_id() }
    }

    /// Wraps `SCNetworkConnectionCreateWithServiceID`.
    pub fn with_service_id(service_id: &str) -> Result<Self> {
        Self::create(service_id, None)
    }

    /// Wraps `SCNetworkConnectionCreateWithServiceID` with an `SCNetworkConnectionCallBack`.
    pub fn with_service_id_and_callback<F>(service_id: &str, callback: F) -> Result<Self>
    where
        F: FnMut(NetworkConnectionStatus) + Send + 'static,
    {
        let state = Arc::new(Mutex::new(CallbackState {
            callback: Box::new(callback),
        }));
        Self::create(service_id, Some(state))
    }

    fn create(service_id: &str, callback: Option<Arc<Mutex<CallbackState>>>) -> Result<Self> {
        let service_id =
            bridge::cstring(service_id, "sc_network_connection_create_with_service_id")?;
        let raw = unsafe {
            ffi::network_connection::sc_network_connection_create_with_service_id(
                service_id.as_ptr(),
                callback
                    .as_ref()
                    .map(|_| network_connection_callback as unsafe extern "C" fn(i32, *mut c_void)),
                callback.as_ref().map_or(std::ptr::null_mut(), |state| {
                    Arc::as_ptr(state).cast_mut().cast::<c_void>()
                }),
            )
        };
        let raw =
            bridge::owned_handle_or_last("sc_network_connection_create_with_service_id", raw)?;
        Ok(Self {
            raw,
            _callback: callback,
        })
    }

    /// Wraps `SCNetworkConnectionCopyUserPreferences`.
    pub fn copy_user_preferences() -> Result<NetworkConnectionUserPreferences> {
        let service_id = bridge::take_optional_string(unsafe {
            ffi::network_connection::sc_network_connection_copy_user_preferences_service_id()
        })
        .ok_or_else(|| {
            crate::SystemConfigurationError::last(
                "sc_network_connection_copy_user_preferences_service_id",
            )
        })?;
        let user_options = unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_connection::sc_network_connection_copy_user_preferences_user_options(),
            )
        }
        .map(PropertyList::from_owned_handle);
        Ok(NetworkConnectionUserPreferences {
            service_id,
            user_options,
        })
    }

    /// Wraps `SCNetworkConnectionCopyServiceID`.
    pub fn service_id(&self) -> Result<Option<String>> {
        Ok(bridge::take_optional_string(unsafe {
            ffi::network_connection::sc_network_connection_copy_service_id(self.raw.as_ptr())
        }))
    }

    /// Wraps `SCNetworkConnectionGetStatus`.
    pub fn status(&self) -> NetworkConnectionStatus {
        NetworkConnectionStatus::from_raw(unsafe {
            ffi::network_connection::sc_network_connection_get_status(self.raw.as_ptr())
        })
    }

    /// Wraps `SCNetworkConnectionCopyExtendedStatus`.
    pub fn extended_status(&self) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_connection::sc_network_connection_copy_extended_status(
                    self.raw.as_ptr(),
                ),
            )
        }
        .map(PropertyList::from_owned_handle)
    }

    /// Wraps `SCNetworkConnectionCopyStatistics`.
    pub fn statistics(&self) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_connection::sc_network_connection_copy_statistics(self.raw.as_ptr()),
            )
        }
        .map(PropertyList::from_owned_handle)
    }

    /// Wraps `SCNetworkConnectionCopyUserOptions`.
    pub fn user_options(&self) -> Option<PropertyList> {
        unsafe {
            bridge::OwnedHandle::from_raw(
                ffi::network_connection::sc_network_connection_copy_user_options(self.raw.as_ptr()),
            )
        }
        .map(PropertyList::from_owned_handle)
    }

    /// Wraps `SCNetworkConnectionStart`.
    pub fn start(&self, user_options: Option<&PropertyList>, linger: bool) -> Result<()> {
        let ok = unsafe {
            ffi::network_connection::sc_network_connection_start(
                self.raw.as_ptr(),
                user_options.map_or(std::ptr::null_mut(), PropertyList::as_ptr),
                u8::from(linger),
            )
        };
        bridge::bool_result("sc_network_connection_start", ok)
    }

    /// Wraps `SCNetworkConnectionStop`.
    pub fn stop(&self, force_disconnect: bool) -> Result<()> {
        let ok = unsafe {
            ffi::network_connection::sc_network_connection_stop(
                self.raw.as_ptr(),
                u8::from(force_disconnect),
            )
        };
        bridge::bool_result("sc_network_connection_stop", ok)
    }

    /// Wraps `SCNetworkConnectionScheduleWithRunLoopCurrent`.
    pub fn schedule_with_run_loop_current(&self) -> Result<()> {
        let ok = unsafe {
            ffi::network_connection::sc_network_connection_schedule_with_run_loop_current(
                self.raw.as_ptr(),
            )
        };
        bridge::bool_result("sc_network_connection_schedule_with_run_loop_current", ok)
    }

    /// Wraps `SCNetworkConnectionUnscheduleFromRunLoopCurrent`.
    pub fn unschedule_from_run_loop_current(&self) -> Result<()> {
        let ok = unsafe {
            ffi::network_connection::sc_network_connection_unschedule_from_run_loop_current(
                self.raw.as_ptr(),
            )
        };
        bridge::bool_result("sc_network_connection_unschedule_from_run_loop_current", ok)
    }

    /// Wraps `SCNetworkConnectionSetDispatchQueueGlobal`.
    pub fn set_dispatch_queue_global(&self) -> Result<()> {
        let ok = unsafe {
            ffi::network_connection::sc_network_connection_set_dispatch_queue_global(
                self.raw.as_ptr(),
            )
        };
        bridge::bool_result("sc_network_connection_set_dispatch_queue_global", ok)
    }

    /// Wraps `SCNetworkConnectionClearDispatchQueue`.
    pub fn clear_dispatch_queue(&self) -> Result<()> {
        let ok = unsafe {
            ffi::network_connection::sc_network_connection_clear_dispatch_queue(self.raw.as_ptr())
        };
        bridge::bool_result("sc_network_connection_clear_dispatch_queue", ok)
    }
}
