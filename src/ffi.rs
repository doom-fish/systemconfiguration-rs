#![allow(non_camel_case_types, non_upper_case_globals)]

use std::ffi::{c_char, c_void};

pub type Boolean = u8;
pub type CFAllocatorRef = *const c_void;
pub type CFArrayRef = *const c_void;
pub type CFPropertyListRef = *const c_void;
pub type CFRunLoopRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type SCDynamicStoreRef = *const c_void;
pub type SCNetworkInterfaceRef = *const c_void;
pub type SCNetworkReachabilityRef = *const c_void;
pub type SCNetworkServiceRef = *const c_void;
pub type SCPreferencesRef = *const c_void;
pub type SCNetworkReachabilityFlags = u32;

pub const kSCNetworkReachabilityFlagsTransientConnection: SCNetworkReachabilityFlags = 1 << 0;
pub const kSCNetworkReachabilityFlagsReachable: SCNetworkReachabilityFlags = 1 << 1;
pub const kSCNetworkReachabilityFlagsConnectionRequired: SCNetworkReachabilityFlags = 1 << 2;
pub const kSCNetworkReachabilityFlagsConnectionOnTraffic: SCNetworkReachabilityFlags = 1 << 3;
pub const kSCNetworkReachabilityFlagsInterventionRequired: SCNetworkReachabilityFlags = 1 << 4;
pub const kSCNetworkReachabilityFlagsConnectionOnDemand: SCNetworkReachabilityFlags = 1 << 5;
pub const kSCNetworkReachabilityFlagsIsLocalAddress: SCNetworkReachabilityFlags = 1 << 16;
pub const kSCNetworkReachabilityFlagsIsDirect: SCNetworkReachabilityFlags = 1 << 17;

#[repr(C)]
pub struct SCNetworkReachabilityContext {
    pub version: isize,
    pub info: *mut c_void,
    pub retain: Option<unsafe extern "C" fn(info: *const c_void) -> *const c_void>,
    pub release: Option<unsafe extern "C" fn(info: *const c_void)>,
    pub copy_description: Option<unsafe extern "C" fn(info: *const c_void) -> CFStringRef>,
}

pub type SCNetworkReachabilityCallBack = Option<
    unsafe extern "C" fn(
        target: SCNetworkReachabilityRef,
        flags: SCNetworkReachabilityFlags,
        info: *mut c_void,
    ),
>;

unsafe extern "C" {
    pub static kCFBooleanTrue: *const c_void;
    pub static kCFBooleanFalse: *const c_void;
    pub static kCFRunLoopDefaultMode: *const c_void;

    pub fn CFArrayCreate(
        allocator: CFAllocatorRef,
        values: *const *const c_void,
        num_values: isize,
        callbacks: *const c_void,
    ) -> CFArrayRef;
    pub fn CFArrayGetCount(the_array: CFArrayRef) -> isize;
    pub fn CFArrayGetValueAtIndex(the_array: CFArrayRef, idx: isize) -> *const c_void;
    pub fn CFCopyDescription(cf: *const c_void) -> CFStringRef;
    pub fn CFRetain(cf: *const c_void) -> *const c_void;
    pub fn CFRelease(cf: *const c_void);
    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    pub fn CFStringCreateWithCString(
        allocator: CFAllocatorRef,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;
    pub fn CFStringGetLength(the_string: CFStringRef) -> isize;
    pub fn CFStringGetMaximumSizeForEncoding(length: isize, encoding: u32) -> isize;
    pub fn CFStringGetCString(
        the_string: CFStringRef,
        buffer: *mut c_char,
        buffer_size: isize,
        encoding: u32,
    ) -> Boolean;

    pub fn SCError() -> i32;
    pub fn SCErrorString(status: i32) -> *const c_char;

    pub fn SCNetworkReachabilityCreateWithAddress(
        allocator: CFAllocatorRef,
        address: *const libc::sockaddr,
    ) -> SCNetworkReachabilityRef;
    pub fn SCNetworkReachabilityCreateWithName(
        allocator: CFAllocatorRef,
        nodename: *const c_char,
    ) -> SCNetworkReachabilityRef;
    pub fn SCNetworkReachabilityGetFlags(
        target: SCNetworkReachabilityRef,
        flags: *mut SCNetworkReachabilityFlags,
    ) -> Boolean;
    pub fn SCNetworkReachabilityScheduleWithRunLoop(
        target: SCNetworkReachabilityRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    ) -> Boolean;
    pub fn SCNetworkReachabilitySetCallback(
        target: SCNetworkReachabilityRef,
        callout: SCNetworkReachabilityCallBack,
        context: *const SCNetworkReachabilityContext,
    ) -> Boolean;
    pub fn SCNetworkReachabilityUnscheduleFromRunLoop(
        target: SCNetworkReachabilityRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    ) -> Boolean;

    pub fn SCDynamicStoreCreate(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        callout: *const c_void,
        context: *const c_void,
    ) -> SCDynamicStoreRef;
    pub fn SCDynamicStoreCopyKeyList(store: SCDynamicStoreRef, pattern: CFStringRef) -> CFArrayRef;
    pub fn SCDynamicStoreCopyValue(store: SCDynamicStoreRef, key: CFStringRef)
        -> CFPropertyListRef;
    pub fn SCDynamicStoreSetNotificationKeys(
        store: SCDynamicStoreRef,
        keys: CFArrayRef,
        patterns: CFArrayRef,
    ) -> Boolean;
    pub fn SCDynamicStoreSetValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;

    pub fn SCNetworkInterfaceCopyAll() -> CFArrayRef;
    pub fn SCNetworkInterfaceGetBSDName(interface: SCNetworkInterfaceRef) -> CFStringRef;
    pub fn SCNetworkInterfaceGetHardwareAddressString(
        interface: SCNetworkInterfaceRef,
    ) -> CFStringRef;
    pub fn SCNetworkInterfaceGetInterfaceType(interface: SCNetworkInterfaceRef) -> CFStringRef;
    pub fn SCNetworkInterfaceGetLocalizedDisplayName(
        interface: SCNetworkInterfaceRef,
    ) -> CFStringRef;

    pub fn SCPreferencesCommitChanges(prefs: SCPreferencesRef) -> Boolean;
    pub fn SCPreferencesCreate(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        prefs_id: CFStringRef,
    ) -> SCPreferencesRef;
    pub fn SCPreferencesLock(prefs: SCPreferencesRef, wait: Boolean) -> Boolean;
    pub fn SCPreferencesSetValue(
        prefs: SCPreferencesRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;

    pub fn SCNetworkServiceCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;
    pub fn SCNetworkServiceGetEnabled(service: SCNetworkServiceRef) -> Boolean;
    pub fn SCNetworkServiceGetServiceID(service: SCNetworkServiceRef) -> CFStringRef;
}
