use std::{ffi::c_void, ffi::CString, net::SocketAddr};

use crate::{
    cf::{current_run_loop, default_run_loop_mode, OwnedCFType},
    error::Result,
    ffi, SystemConfigurationError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct ReachabilityFlags(pub ffi::SCNetworkReachabilityFlags);

impl ReachabilityFlags {
    pub fn bits(self) -> ffi::SCNetworkReachabilityFlags {
        self.0
    }

    pub fn is_transient_connection(self) -> bool {
        self.0 & ffi::kSCNetworkReachabilityFlagsTransientConnection != 0
    }

    pub fn is_reachable(self) -> bool {
        self.0 & ffi::kSCNetworkReachabilityFlagsReachable != 0
    }

    pub fn needs_connection(self) -> bool {
        self.0 & ffi::kSCNetworkReachabilityFlagsConnectionRequired != 0
    }

    pub fn is_connection_on_traffic(self) -> bool {
        self.0 & ffi::kSCNetworkReachabilityFlagsConnectionOnTraffic != 0
    }

    pub fn needs_intervention(self) -> bool {
        self.0 & ffi::kSCNetworkReachabilityFlagsInterventionRequired != 0
    }

    pub fn is_connection_on_demand(self) -> bool {
        self.0 & ffi::kSCNetworkReachabilityFlagsConnectionOnDemand != 0
    }

    pub fn is_local_address(self) -> bool {
        self.0 & ffi::kSCNetworkReachabilityFlagsIsLocalAddress != 0
    }

    pub fn is_direct(self) -> bool {
        self.0 & ffi::kSCNetworkReachabilityFlagsIsDirect != 0
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
        if labels.is_empty() {
            write!(f, "0x{:x}", self.bits())
        } else {
            write!(f, "{} (0x{:x})", labels.join("|"), self.bits())
        }
    }
}

struct CallbackState {
    callback: Box<dyn FnMut(ReachabilityFlags)>,
}

unsafe extern "C" fn reachability_callback(
    _target: ffi::SCNetworkReachabilityRef,
    flags: ffi::SCNetworkReachabilityFlags,
    info: *mut c_void,
) {
    if info.is_null() {
        return;
    }

    let state = &mut *info.cast::<CallbackState>();
    (state.callback)(ReachabilityFlags(flags));
}

pub struct Reachability {
    raw: OwnedCFType,
    callback: Option<Box<CallbackState>>,
    scheduled_with_current_run_loop: bool,
}

impl Reachability {
    pub fn with_name(name: &str) -> Result<Self> {
        let name = CString::new(name).map_err(|_| {
            SystemConfigurationError::null(
                "SCNetworkReachabilityCreateWithName",
                "host names cannot contain interior NUL bytes",
            )
        })?;
        let raw =
            unsafe { ffi::SCNetworkReachabilityCreateWithName(std::ptr::null(), name.as_ptr()) };
        let raw = unsafe { OwnedCFType::from_create_rule(raw) }
            .ok_or_else(|| SystemConfigurationError::last("SCNetworkReachabilityCreateWithName"))?;
        Ok(Self {
            raw,
            callback: None,
            scheduled_with_current_run_loop: false,
        })
    }

    pub fn with_address(address: SocketAddr) -> Result<Self> {
        let raw = match address {
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
                    ffi::SCNetworkReachabilityCreateWithAddress(
                        std::ptr::null(),
                        std::ptr::from_ref(&storage).cast::<libc::sockaddr>(),
                    )
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
                    ffi::SCNetworkReachabilityCreateWithAddress(
                        std::ptr::null(),
                        std::ptr::from_ref(&storage).cast::<libc::sockaddr>(),
                    )
                }
            }
        };

        let raw = unsafe { OwnedCFType::from_create_rule(raw) }.ok_or_else(|| {
            SystemConfigurationError::last("SCNetworkReachabilityCreateWithAddress")
        })?;
        Ok(Self {
            raw,
            callback: None,
            scheduled_with_current_run_loop: false,
        })
    }

    pub fn flags(&self) -> Result<ReachabilityFlags> {
        let mut flags = 0_u32;
        let ok = unsafe { ffi::SCNetworkReachabilityGetFlags(self.raw.as_ptr(), &mut flags) };
        if ok == 0 {
            Err(SystemConfigurationError::last(
                "SCNetworkReachabilityGetFlags",
            ))
        } else {
            Ok(ReachabilityFlags(flags))
        }
    }

    pub fn set_callback<F>(&mut self, callback: F) -> Result<()>
    where
        F: FnMut(ReachabilityFlags) + 'static,
    {
        let mut callback = Box::new(CallbackState {
            callback: Box::new(callback),
        });
        let context = ffi::SCNetworkReachabilityContext {
            version: 0,
            info: std::ptr::from_mut(&mut *callback).cast::<c_void>(),
            retain: None,
            release: None,
            copy_description: None,
        };
        let ok = unsafe {
            ffi::SCNetworkReachabilitySetCallback(
                self.raw.as_ptr(),
                Some(reachability_callback),
                &context,
            )
        };
        if ok == 0 {
            Err(SystemConfigurationError::last(
                "SCNetworkReachabilitySetCallback",
            ))
        } else {
            self.callback = Some(callback);
            Ok(())
        }
    }

    pub fn clear_callback(&mut self) -> Result<()> {
        let ok = unsafe {
            ffi::SCNetworkReachabilitySetCallback(self.raw.as_ptr(), None, std::ptr::null())
        };
        if ok == 0 {
            Err(SystemConfigurationError::last(
                "SCNetworkReachabilitySetCallback",
            ))
        } else {
            self.callback = None;
            Ok(())
        }
    }

    pub fn schedule_with_run_loop_current(&mut self) -> Result<()> {
        let ok = unsafe {
            ffi::SCNetworkReachabilityScheduleWithRunLoop(
                self.raw.as_ptr(),
                current_run_loop(),
                default_run_loop_mode(),
            )
        };
        if ok == 0 {
            Err(SystemConfigurationError::last(
                "SCNetworkReachabilityScheduleWithRunLoop",
            ))
        } else {
            self.scheduled_with_current_run_loop = true;
            Ok(())
        }
    }
}

impl Drop for Reachability {
    fn drop(&mut self) {
        if self.scheduled_with_current_run_loop {
            let _ = unsafe {
                ffi::SCNetworkReachabilityUnscheduleFromRunLoop(
                    self.raw.as_ptr(),
                    current_run_loop(),
                    default_run_loop_mode(),
                )
            };
        }
        if self.callback.is_some() {
            let _ = unsafe {
                ffi::SCNetworkReachabilitySetCallback(self.raw.as_ptr(), None, std::ptr::null())
            };
        }
    }
}
