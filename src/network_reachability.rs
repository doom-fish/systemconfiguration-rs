use std::{
    ffi::c_void,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{bridge, error::Result, ffi, SystemConfigurationError};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
/// Wraps `SCNetworkReachabilityFlags`.
pub struct ReachabilityFlags(
    /// Wraps the raw `SCNetworkReachabilityFlags` bitfield.
    pub u32,
);

impl ReachabilityFlags {
    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn bits(self) -> u32 {
        self.0
    }

    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn is_transient_connection(self) -> bool {
        self.0 & (1 << 0) != 0
    }

    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn is_reachable(self) -> bool {
        self.0 & (1 << 1) != 0
    }

    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn needs_connection(self) -> bool {
        self.0 & (1 << 2) != 0
    }

    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn is_connection_on_traffic(self) -> bool {
        self.0 & (1 << 3) != 0
    }

    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn needs_intervention(self) -> bool {
        self.0 & (1 << 4) != 0
    }

    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn is_connection_on_demand(self) -> bool {
        self.0 & (1 << 5) != 0
    }

    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn is_local_address(self) -> bool {
        self.0 & (1 << 16) != 0
    }

    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn is_direct(self) -> bool {
        self.0 & (1 << 17) != 0
    }

    /// Wraps a helper on `SCNetworkReachabilityFlags`.
    pub fn is_wwan(self) -> bool {
        self.0 & (1 << 18) != 0
    }
}

impl std::fmt::Display for ReachabilityFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut labels = Vec::new();
        if self.is_transient_connection() {
            labels.push("transient");
        }
        if self.is_reachable() {
            labels.push("reachable");
        }
        if self.needs_connection() {
            labels.push("needs-connection");
        }
        if self.is_connection_on_traffic() {
            labels.push("on-traffic");
        }
        if self.needs_intervention() {
            labels.push("needs-intervention");
        }
        if self.is_connection_on_demand() {
            labels.push("on-demand");
        }
        if self.is_local_address() {
            labels.push("local-address");
        }
        if self.is_direct() {
            labels.push("direct");
        }
        if self.is_wwan() {
            labels.push("wwan");
        }
        if labels.is_empty() {
            write!(f, "0x{:x}", self.bits())
        } else {
            write!(f, "{} (0x{:x})", labels.join("|"), self.bits())
        }
    }
}

struct LocalCallbackState {
    callback: Box<dyn FnMut(ReachabilityFlags)>,
}

struct SendCallbackState {
    callback: Box<dyn FnMut(ReachabilityFlags) + Send>,
}

enum RegisteredCallback {
    Local {
        _state: Box<LocalCallbackState>,
    },
    Send {
        _state: Arc<Mutex<SendCallbackState>>,
    },
}

unsafe extern "C" fn reachability_callback_local(flags: u32, info: *mut c_void) {
    if info.is_null() {
        return;
    }

    let state = unsafe { &mut *info.cast::<LocalCallbackState>() };
    (state.callback)(ReachabilityFlags(flags));
}

unsafe extern "C" fn reachability_callback_send(flags: u32, info: *mut c_void) {
    if info.is_null() {
        return;
    }

    let mutex = unsafe { &*info.cast::<Mutex<SendCallbackState>>() };
    if let Ok(mut state) = mutex.lock() {
        (state.callback)(ReachabilityFlags(flags));
    }
}

/// Wraps `SCNetworkReachabilityRef`.
pub struct Reachability {
    raw: bridge::OwnedHandle,
    callback: Option<RegisteredCallback>,
    scheduled_with_current_run_loop: bool,
    dispatch_queue_active: bool,
}

/// Alias for the `SCNetworkReachabilityRef` wrapper.
pub type NetworkReachability = Reachability;

impl Reachability {
    /// Wraps `SCReachabilityGetTypeID`.
    pub fn type_id() -> u64 {
        unsafe { ffi::network_reachability::sc_reachability_get_type_id() }
    }

    /// Wraps `SCReachabilityCreateWithName`.
    pub fn with_name(name: &str) -> Result<Self> {
        let name = bridge::cstring(name, "sc_reachability_create_with_name")?;
        let raw =
            unsafe { ffi::network_reachability::sc_reachability_create_with_name(name.as_ptr()) };
        let raw = bridge::owned_handle_or_last("sc_reachability_create_with_name", raw)?;
        Ok(Self {
            raw,
            callback: None,
            scheduled_with_current_run_loop: false,
            dispatch_queue_active: false,
        })
    }

    /// Wraps `SCReachabilityCreateWithAddress`.
    pub fn with_address(address: SocketAddr) -> Result<Self> {
        let storage = socket_addr_to_bytes(address);
        let raw = unsafe {
            ffi::network_reachability::sc_reachability_create_with_address(
                storage.as_ptr(),
                isize::try_from(storage.len()).expect("socket address length exceeded isize"),
            )
        };
        let raw = bridge::owned_handle_or_last("sc_reachability_create_with_address", raw)?;
        Ok(Self {
            raw,
            callback: None,
            scheduled_with_current_run_loop: false,
            dispatch_queue_active: false,
        })
    }

    /// Wraps `SCReachabilityCreateWithAddressPair`.
    pub fn with_address_pair(
        local_address: Option<SocketAddr>,
        remote_address: Option<SocketAddr>,
    ) -> Result<Self> {
        let local = local_address.map(socket_addr_to_bytes);
        let remote = remote_address.map(socket_addr_to_bytes);
        let raw = unsafe {
            ffi::network_reachability::sc_reachability_create_with_address_pair(
                local.as_ref().map_or(std::ptr::null(), Vec::as_ptr),
                local.as_ref().map_or(0, |value| {
                    isize::try_from(value.len()).expect("socket address length exceeded isize")
                }),
                remote.as_ref().map_or(std::ptr::null(), Vec::as_ptr),
                remote.as_ref().map_or(0, |value| {
                    isize::try_from(value.len()).expect("socket address length exceeded isize")
                }),
            )
        };
        let raw = bridge::owned_handle_or_last("sc_reachability_create_with_address_pair", raw)?;
        Ok(Self {
            raw,
            callback: None,
            scheduled_with_current_run_loop: false,
            dispatch_queue_active: false,
        })
    }

    /// Wraps `SCReachabilityGetFlags`.
    pub fn flags(&self) -> Result<ReachabilityFlags> {
        let mut flags = 0_u32;
        let ok = unsafe {
            ffi::network_reachability::sc_reachability_get_flags(self.raw.as_ptr(), &mut flags)
        };
        bridge::bool_result("sc_reachability_get_flags", ok)?;
        Ok(ReachabilityFlags(flags))
    }

    /// Wraps a helper on `SCNetworkReachabilityRef`.
    pub fn set_callback<F>(&mut self, callback: F) -> Result<()>
    where
        F: FnMut(ReachabilityFlags) + 'static,
    {
        if self.dispatch_queue_active {
            return Err(SystemConfigurationError::null(
                "sc_reachability_set_callback",
                "dispatch queues require callbacks registered via Reachability::set_callback_send; clear the dispatch queue first",
            ));
        }

        let mut callback = Box::new(LocalCallbackState {
            callback: Box::new(callback),
        });
        self.set_registered_callback(
            Some(reachability_callback_local),
            std::ptr::from_mut(&mut *callback).cast::<c_void>(),
            Some(RegisteredCallback::Local { _state: callback }),
        )
    }

    /// Wraps a helper on `SCNetworkReachabilityRef`.
    pub fn set_callback_send<F>(&mut self, callback: F) -> Result<()>
    where
        F: FnMut(ReachabilityFlags) + Send + 'static,
    {
        let callback = Arc::new(Mutex::new(SendCallbackState {
            callback: Box::new(callback),
        }));
        self.set_registered_callback(
            Some(reachability_callback_send),
            Arc::as_ptr(&callback).cast_mut().cast::<c_void>(),
            Some(RegisteredCallback::Send { _state: callback }),
        )
    }

    /// Wraps a helper on `SCNetworkReachabilityRef`.
    pub fn clear_callback(&mut self) -> Result<()> {
        if self.dispatch_queue_active {
            self.clear_dispatch_queue()?;
        }
        self.set_registered_callback(None, std::ptr::null_mut(), None)
    }

    /// Wraps `SCReachabilityScheduleWithRunLoopCurrent`.
    pub fn schedule_with_run_loop_current(&mut self) -> Result<()> {
        let ok = unsafe {
            ffi::network_reachability::sc_reachability_schedule_with_run_loop_current(
                self.raw.as_ptr(),
            )
        };
        bridge::bool_result("sc_reachability_schedule_with_run_loop_current", ok)?;
        self.scheduled_with_current_run_loop = true;
        Ok(())
    }

    /// Wraps `SCReachabilityUnscheduleFromRunLoopCurrent`.
    pub fn unschedule_from_run_loop_current(&mut self) -> Result<()> {
        let ok = unsafe {
            ffi::network_reachability::sc_reachability_unschedule_from_run_loop_current(
                self.raw.as_ptr(),
            )
        };
        bridge::bool_result("sc_reachability_unschedule_from_run_loop_current", ok)?;
        self.scheduled_with_current_run_loop = false;
        Ok(())
    }

    /// Wraps `SCReachabilitySetDispatchQueueGlobal`.
    pub fn set_dispatch_queue_global(&mut self) -> Result<()> {
        if matches!(self.callback, Some(RegisteredCallback::Local { .. })) {
            return Err(SystemConfigurationError::null(
                "sc_reachability_set_dispatch_queue_global",
                "dispatch queues require callbacks registered via Reachability::set_callback_send",
            ));
        }

        let ok = unsafe {
            ffi::network_reachability::sc_reachability_set_dispatch_queue_global(self.raw.as_ptr())
        };
        bridge::bool_result("sc_reachability_set_dispatch_queue_global", ok)?;
        self.dispatch_queue_active = true;
        Ok(())
    }

    /// Wraps `SCReachabilityClearDispatchQueue`.
    pub fn clear_dispatch_queue(&mut self) -> Result<()> {
        let ok = unsafe {
            ffi::network_reachability::sc_reachability_clear_dispatch_queue(self.raw.as_ptr())
        };
        bridge::bool_result("sc_reachability_clear_dispatch_queue", ok)?;
        self.dispatch_queue_active = false;
        Ok(())
    }

    fn set_registered_callback(
        &mut self,
        callback: ffi::network_reachability::ReachabilityCallback,
        info: *mut c_void,
        registered: Option<RegisteredCallback>,
    ) -> Result<()> {
        let ok = unsafe {
            ffi::network_reachability::sc_reachability_set_callback(
                self.raw.as_ptr(),
                callback,
                info,
            )
        };
        bridge::bool_result("sc_reachability_set_callback", ok)?;
        self.callback = registered;
        Ok(())
    }
}

impl Drop for Reachability {
    fn drop(&mut self) {
        if self.dispatch_queue_active {
            let _ = unsafe {
                ffi::network_reachability::sc_reachability_clear_dispatch_queue(self.raw.as_ptr())
            };
        }
        if self.scheduled_with_current_run_loop {
            let _ = unsafe {
                ffi::network_reachability::sc_reachability_unschedule_from_run_loop_current(
                    self.raw.as_ptr(),
                )
            };
        }
        if self.callback.is_some() {
            let _ = unsafe {
                ffi::network_reachability::sc_reachability_set_callback(
                    self.raw.as_ptr(),
                    None,
                    std::ptr::null_mut(),
                )
            };
        }
    }
}

fn socket_addr_to_bytes(address: SocketAddr) -> Vec<u8> {
    match address {
        SocketAddr::V4(address) => {
            let mut storage: libc::sockaddr_in = unsafe { std::mem::zeroed() };
            storage.sin_len = u8::try_from(std::mem::size_of::<libc::sockaddr_in>())
                .expect("sockaddr_in length exceeds u8");
            storage.sin_family = u8::try_from(libc::AF_INET).expect("AF_INET exceeds u8");
            storage.sin_port = address.port().to_be();
            storage.sin_addr = libc::in_addr {
                s_addr: u32::from_ne_bytes(address.ip().octets()),
            };
            unsafe {
                std::slice::from_raw_parts(
                    std::ptr::from_ref(&storage).cast::<u8>(),
                    std::mem::size_of::<libc::sockaddr_in>(),
                )
                .to_vec()
            }
        }
        SocketAddr::V6(address) => {
            let mut storage: libc::sockaddr_in6 = unsafe { std::mem::zeroed() };
            storage.sin6_len = u8::try_from(std::mem::size_of::<libc::sockaddr_in6>())
                .expect("sockaddr_in6 length exceeds u8");
            storage.sin6_family = u8::try_from(libc::AF_INET6).expect("AF_INET6 exceeds u8");
            storage.sin6_port = address.port().to_be();
            storage.sin6_flowinfo = address.flowinfo();
            storage.sin6_scope_id = address.scope_id();
            storage.sin6_addr = libc::in6_addr {
                s6_addr: address.ip().octets(),
            };
            unsafe {
                std::slice::from_raw_parts(
                    std::ptr::from_ref(&storage).cast::<u8>(),
                    std::mem::size_of::<libc::sockaddr_in6>(),
                )
                .to_vec()
            }
        }
    }
}
