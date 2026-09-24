use std::{ffi::c_void, net::SocketAddr, sync::Mutex};

use apple_cf::{cf::CFRunLoop, dispatch_queue::DispatchQueue};
use doom_fish_utils::callback_context::CallbackContext;

use crate::{bridge, error::Result, ffi, RunLoopMode};

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

type ReachabilityCallback = bridge::CallbackSlot<dyn FnMut(ReachabilityFlags) + Send>;
type ReachabilityCallbackContext = CallbackContext<ReachabilityCallback>;

unsafe extern "C" fn reachability_callback(flags: u32, info: *mut c_void) {
    unsafe {
        ReachabilityCallbackContext::with(info, "reachability_callback", |slot| {
            bridge::with_callback(slot, |callback| callback(ReachabilityFlags(flags)));
        });
    }
}

/// Wraps `SCNetworkReachabilityRef`.
pub struct Reachability {
    raw: bridge::OwnedHandle,
    callback: Option<ReachabilityCallbackContext>,
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
        F: FnMut(ReachabilityFlags) + Send + 'static,
    {
        let context = ReachabilityCallbackContext::new(Mutex::new(Some(Box::new(callback))));
        let ok = unsafe {
            ffi::network_reachability::sc_reachability_set_callback(
                self.raw.as_ptr(),
                Some(reachability_callback),
                context.as_ptr(),
                Some(ReachabilityCallbackContext::RETAIN),
                Some(ReachabilityCallbackContext::RELEASE),
            )
        };
        bridge::bool_result("sc_reachability_set_callback", ok)?;
        let previous = self.callback.replace(context);
        if let Some(previous) = &previous {
            bridge::retire_callback(previous);
        }
        drop(previous);
        Ok(())
    }

    /// Wraps a helper on `SCNetworkReachabilityRef`.
    pub fn clear_callback(&mut self) -> Result<()> {
        if self.dispatch_queue_active {
            self.clear_dispatch_queue()?;
        }
        if let Some(current) = &self.callback {
            bridge::retire_callback(current);
        }
        let ok = unsafe {
            ffi::network_reachability::sc_reachability_set_callback(
                self.raw.as_ptr(),
                None,
                std::ptr::null_mut(),
                None,
                None,
            )
        };
        bridge::bool_result("sc_reachability_set_callback", ok)?;
        self.callback = None;
        Ok(())
    }

    pub fn schedule_with_run_loop(
        &mut self,
        run_loop: &CFRunLoop,
        mode: RunLoopMode<'_>,
    ) -> Result<()> {
        let ok = mode.with_raw(|mode| unsafe {
            ffi::network_reachability::sc_reachability_schedule_with_run_loop(
                self.raw.as_ptr(),
                run_loop.as_ptr(),
                mode,
            )
        });
        bridge::bool_result("sc_reachability_schedule_with_run_loop", ok)
    }

    pub fn unschedule_from_run_loop(
        &mut self,
        run_loop: &CFRunLoop,
        mode: RunLoopMode<'_>,
    ) -> Result<()> {
        let ok = mode.with_raw(|mode| unsafe {
            ffi::network_reachability::sc_reachability_unschedule_from_run_loop(
                self.raw.as_ptr(),
                run_loop.as_ptr(),
                mode,
            )
        });
        bridge::bool_result("sc_reachability_unschedule_from_run_loop", ok)
    }

    pub fn set_dispatch_queue(&mut self, queue: &DispatchQueue) -> Result<()> {
        let ok = unsafe {
            ffi::network_reachability::sc_reachability_set_dispatch_queue(
                self.raw.as_ptr(),
                queue.as_ptr().cast_mut(),
            )
        };
        bridge::bool_result("sc_reachability_set_dispatch_queue", ok)?;
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
}

impl Drop for Reachability {
    fn drop(&mut self) {
        if let Some(callback) = &self.callback {
            bridge::retire_callback(callback);
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use apple_cf::{
        cf::CFRunLoop,
        dispatch_queue::{DispatchQoS, DispatchQueue},
    };

    use super::Reachability;
    use crate::{
        bridge::test_support::{run_loop_mode_is_empty, wait_for, InFlightCall},
        RunLoopMode,
    };

    #[test]
    fn dropping_a_reachability_unschedules_it_from_every_run_loop_mode() {
        let witness = Arc::new(());
        let captured = Arc::clone(&witness);
        let mut reachability = Reachability::with_name("localhost").expect("reachability");
        reachability
            .set_callback(move |_| {
                let _ = &captured;
            })
            .expect("callback");
        let mode = "systemconfiguration-rs.unit-reach-mode";
        reachability
            .schedule_with_run_loop(&CFRunLoop::current(), RunLoopMode::Named(mode))
            .expect("schedule");
        assert!(!run_loop_mode_is_empty(mode));

        drop(reachability);
        assert!(run_loop_mode_is_empty(mode));
        assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    }

    #[test]
    fn dropping_a_reachability_on_a_queue_waits_for_sc_to_release_the_callback() {
        let witness = Arc::new(());
        let captured = Arc::clone(&witness);
        let mut reachability = Reachability::with_name("localhost").expect("reachability");
        reachability
            .set_callback(move |_| {
                let _ = &captured;
            })
            .expect("callback");
        let queue = DispatchQueue::new("systemconfiguration-rs.unit-reach", DispatchQoS::Utility);
        reachability
            .set_dispatch_queue(&queue)
            .expect("dispatch queue");

        let call = InFlightCall::hold(reachability.callback.as_ref().expect("callback"));
        drop(reachability);
        assert_eq!(Arc::strong_count(&witness), 2);

        call.finish();
        assert!(wait_for(|| Arc::strong_count(&witness) == 1));
    }
}
