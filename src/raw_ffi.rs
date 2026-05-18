#![allow(non_camel_case_types, non_upper_case_globals)]

use std::ffi::{c_char, c_void};

pub use apple_cf::raw::{
    Boolean, CFAllocatorRef, CFArrayRef, CFDataRef, CFDateRef, CFDictionaryRef,
    CFPropertyListRef, CFRunLoopRef, CFStringRef,
};
pub type SCDynamicStoreRef = *const c_void;
pub type SCNetworkInterfaceRef = *const c_void;
pub type SCNetworkProtocolRef = *const c_void;
pub type SCNetworkReachabilityRef = *const c_void;
pub type SCNetworkServiceRef = *const c_void;
pub type SCNetworkSetRef = *const c_void;
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
pub const kSCNetworkReachabilityFlagsIsWWAN: SCNetworkReachabilityFlags = 1 << 18;

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

    pub fn SCDynamicStoreCreate(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        callout: *const c_void,
        context: *const c_void,
    ) -> SCDynamicStoreRef;
    pub fn SCDynamicStoreCopyKeyList(store: SCDynamicStoreRef, pattern: CFStringRef) -> CFArrayRef;
    pub fn SCDynamicStoreCopyValue(store: SCDynamicStoreRef, key: CFStringRef)
        -> CFPropertyListRef;
    pub fn SCDynamicStoreCopyMultiple(
        store: SCDynamicStoreRef,
        keys: CFArrayRef,
        patterns: CFArrayRef,
    ) -> CFDictionaryRef;
    pub fn SCDynamicStoreAddValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
    pub fn SCDynamicStoreAddTemporaryValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
    pub fn SCDynamicStoreSetValue(
        store: SCDynamicStoreRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
    pub fn SCDynamicStoreRemoveValue(store: SCDynamicStoreRef, key: CFStringRef) -> Boolean;
    pub fn SCDynamicStoreNotifyValue(store: SCDynamicStoreRef, key: CFStringRef) -> Boolean;
    pub fn SCDynamicStoreSetNotificationKeys(
        store: SCDynamicStoreRef,
        keys: CFArrayRef,
        patterns: CFArrayRef,
    ) -> Boolean;
    pub fn SCDynamicStoreCopyNotifiedKeys(store: SCDynamicStoreRef) -> CFArrayRef;
    pub fn SCDynamicStoreCopyComputerName(
        store: SCDynamicStoreRef,
        encoding: *mut u32,
    ) -> CFStringRef;
    pub fn SCDynamicStoreCopyConsoleUser(
        store: SCDynamicStoreRef,
        uid: *mut libc::uid_t,
        gid: *mut libc::gid_t,
    ) -> CFStringRef;
    pub fn SCDynamicStoreCopyLocalHostName(store: SCDynamicStoreRef) -> CFStringRef;
    pub fn SCDynamicStoreCopyLocation(store: SCDynamicStoreRef) -> CFStringRef;
    pub fn SCDynamicStoreCopyProxies(store: SCDynamicStoreRef) -> CFDictionaryRef;
    pub fn SCDynamicStoreCopyDHCPInfo(
        store: SCDynamicStoreRef,
        service_id: CFStringRef,
    ) -> CFDictionaryRef;
    pub fn SCDynamicStoreKeyCreateNetworkGlobalEntity(
        allocator: CFAllocatorRef,
        domain: CFStringRef,
        entity: CFStringRef,
    ) -> CFStringRef;
    pub fn SCDynamicStoreKeyCreateNetworkInterface(
        allocator: CFAllocatorRef,
        domain: CFStringRef,
    ) -> CFStringRef;
    pub fn SCDynamicStoreKeyCreateNetworkInterfaceEntity(
        allocator: CFAllocatorRef,
        domain: CFStringRef,
        ifname: CFStringRef,
        entity: CFStringRef,
    ) -> CFStringRef;
    pub fn SCDynamicStoreKeyCreateNetworkServiceEntity(
        allocator: CFAllocatorRef,
        domain: CFStringRef,
        service_id: CFStringRef,
        entity: CFStringRef,
    ) -> CFStringRef;
    pub fn SCDynamicStoreKeyCreateComputerName(allocator: CFAllocatorRef) -> CFStringRef;
    pub fn SCDynamicStoreKeyCreateConsoleUser(allocator: CFAllocatorRef) -> CFStringRef;
    pub fn SCDynamicStoreKeyCreateHostNames(allocator: CFAllocatorRef) -> CFStringRef;
    pub fn SCDynamicStoreKeyCreateLocation(allocator: CFAllocatorRef) -> CFStringRef;
    pub fn SCDynamicStoreKeyCreateProxies(allocator: CFAllocatorRef) -> CFStringRef;

    pub fn SCPreferencesCreate(
        allocator: CFAllocatorRef,
        name: CFStringRef,
        prefs_id: CFStringRef,
    ) -> SCPreferencesRef;
    pub fn SCPreferencesLock(prefs: SCPreferencesRef, wait: Boolean) -> Boolean;
    pub fn SCPreferencesCommitChanges(prefs: SCPreferencesRef) -> Boolean;
    pub fn SCPreferencesApplyChanges(prefs: SCPreferencesRef) -> Boolean;
    pub fn SCPreferencesUnlock(prefs: SCPreferencesRef) -> Boolean;
    pub fn SCPreferencesGetSignature(prefs: SCPreferencesRef) -> CFDataRef;
    pub fn SCPreferencesCopyKeyList(prefs: SCPreferencesRef) -> CFArrayRef;
    pub fn SCPreferencesGetValue(prefs: SCPreferencesRef, key: CFStringRef) -> CFPropertyListRef;
    pub fn SCPreferencesAddValue(
        prefs: SCPreferencesRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
    pub fn SCPreferencesSetValue(
        prefs: SCPreferencesRef,
        key: CFStringRef,
        value: CFPropertyListRef,
    ) -> Boolean;
    pub fn SCPreferencesRemoveValue(prefs: SCPreferencesRef, key: CFStringRef) -> Boolean;
    pub fn SCPreferencesSynchronize(prefs: SCPreferencesRef);
    pub fn SCPreferencesPathCreateUniqueChild(
        prefs: SCPreferencesRef,
        prefix: CFStringRef,
    ) -> CFStringRef;
    pub fn SCPreferencesPathGetValue(prefs: SCPreferencesRef, path: CFStringRef)
        -> CFDictionaryRef;
    pub fn SCPreferencesPathGetLink(prefs: SCPreferencesRef, path: CFStringRef) -> CFStringRef;
    pub fn SCPreferencesPathSetValue(
        prefs: SCPreferencesRef,
        path: CFStringRef,
        value: CFDictionaryRef,
    ) -> Boolean;
    pub fn SCPreferencesPathSetLink(
        prefs: SCPreferencesRef,
        path: CFStringRef,
        link: CFStringRef,
    ) -> Boolean;
    pub fn SCPreferencesPathRemoveValue(prefs: SCPreferencesRef, path: CFStringRef) -> Boolean;
    pub fn SCPreferencesSetComputerName(
        prefs: SCPreferencesRef,
        name: CFStringRef,
        name_encoding: u32,
    ) -> Boolean;
    pub fn SCPreferencesSetLocalHostName(prefs: SCPreferencesRef, name: CFStringRef) -> Boolean;

    pub fn SCNetworkInterfaceCopyAll() -> CFArrayRef;
    pub fn SCNetworkInterfaceGetBSDName(interface: SCNetworkInterfaceRef) -> CFStringRef;
    pub fn SCNetworkInterfaceGetConfiguration(interface: SCNetworkInterfaceRef) -> CFDictionaryRef;
    pub fn SCNetworkInterfaceGetExtendedConfiguration(
        interface: SCNetworkInterfaceRef,
        extended_type: CFStringRef,
    ) -> CFDictionaryRef;
    pub fn SCNetworkInterfaceGetHardwareAddressString(
        interface: SCNetworkInterfaceRef,
    ) -> CFStringRef;
    pub fn SCNetworkInterfaceGetInterface(
        interface: SCNetworkInterfaceRef,
    ) -> SCNetworkInterfaceRef;
    pub fn SCNetworkInterfaceGetInterfaceType(interface: SCNetworkInterfaceRef) -> CFStringRef;
    pub fn SCNetworkInterfaceGetLocalizedDisplayName(
        interface: SCNetworkInterfaceRef,
    ) -> CFStringRef;
    pub fn SCNetworkInterfaceGetSupportedInterfaceTypes(
        interface: SCNetworkInterfaceRef,
    ) -> CFArrayRef;
    pub fn SCNetworkInterfaceGetSupportedProtocolTypes(
        interface: SCNetworkInterfaceRef,
    ) -> CFArrayRef;
    pub fn SCNetworkInterfaceCreateWithInterface(
        interface: SCNetworkInterfaceRef,
        interface_type: CFStringRef,
    ) -> SCNetworkInterfaceRef;
    pub fn SCNetworkInterfaceCopyMTU(
        interface: SCNetworkInterfaceRef,
        current: *mut i32,
        minimum: *mut i32,
        maximum: *mut i32,
    ) -> Boolean;
    pub fn SCNetworkInterfaceForceConfigurationRefresh(interface: SCNetworkInterfaceRef)
        -> Boolean;

    pub fn SCNetworkProtocolGetConfiguration(protocol: SCNetworkProtocolRef) -> CFDictionaryRef;
    pub fn SCNetworkProtocolGetEnabled(protocol: SCNetworkProtocolRef) -> Boolean;
    pub fn SCNetworkProtocolGetProtocolType(protocol: SCNetworkProtocolRef) -> CFStringRef;
    pub fn SCNetworkProtocolSetConfiguration(
        protocol: SCNetworkProtocolRef,
        config: CFDictionaryRef,
    ) -> Boolean;
    pub fn SCNetworkProtocolSetEnabled(protocol: SCNetworkProtocolRef, enabled: Boolean)
        -> Boolean;

    pub fn SCNetworkServiceCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;
    pub fn SCNetworkServiceCopyProtocols(service: SCNetworkServiceRef) -> CFArrayRef;
    pub fn SCNetworkServiceCopyProtocol(
        service: SCNetworkServiceRef,
        protocol_type: CFStringRef,
    ) -> SCNetworkProtocolRef;
    pub fn SCNetworkServiceAddProtocolType(
        service: SCNetworkServiceRef,
        protocol_type: CFStringRef,
    ) -> Boolean;
    pub fn SCNetworkServiceEstablishDefaultConfiguration(service: SCNetworkServiceRef) -> Boolean;
    pub fn SCNetworkServiceGetEnabled(service: SCNetworkServiceRef) -> Boolean;
    pub fn SCNetworkServiceGetInterface(service: SCNetworkServiceRef) -> SCNetworkInterfaceRef;
    pub fn SCNetworkServiceGetName(service: SCNetworkServiceRef) -> CFStringRef;
    pub fn SCNetworkServiceGetServiceID(service: SCNetworkServiceRef) -> CFStringRef;
    pub fn SCNetworkServiceRemove(service: SCNetworkServiceRef) -> Boolean;
    pub fn SCNetworkServiceRemoveProtocolType(
        service: SCNetworkServiceRef,
        protocol_type: CFStringRef,
    ) -> Boolean;
    pub fn SCNetworkServiceSetEnabled(service: SCNetworkServiceRef, enabled: Boolean) -> Boolean;
    pub fn SCNetworkServiceSetName(service: SCNetworkServiceRef, name: CFStringRef) -> Boolean;

    pub fn SCNetworkSetCopyAll(prefs: SCPreferencesRef) -> CFArrayRef;
    pub fn SCNetworkSetCopyCurrent(prefs: SCPreferencesRef) -> SCNetworkSetRef;
    pub fn SCNetworkSetCopyServices(set: SCNetworkSetRef) -> CFArrayRef;
    pub fn SCNetworkSetGetName(set: SCNetworkSetRef) -> CFStringRef;
    pub fn SCNetworkSetGetSetID(set: SCNetworkSetRef) -> CFStringRef;
    pub fn SCNetworkSetGetServiceOrder(set: SCNetworkSetRef) -> CFArrayRef;
    pub fn SCNetworkSetContainsInterface(
        set: SCNetworkSetRef,
        interface: SCNetworkInterfaceRef,
    ) -> Boolean;
    pub fn SCNetworkSetAddService(set: SCNetworkSetRef, service: SCNetworkServiceRef) -> Boolean;
    pub fn SCNetworkSetRemove(set: SCNetworkSetRef) -> Boolean;
    pub fn SCNetworkSetRemoveService(set: SCNetworkSetRef, service: SCNetworkServiceRef)
        -> Boolean;
    pub fn SCNetworkSetSetCurrent(set: SCNetworkSetRef) -> Boolean;
    pub fn SCNetworkSetSetName(set: SCNetworkSetRef, name: CFStringRef) -> Boolean;
    pub fn SCNetworkSetSetServiceOrder(set: SCNetworkSetRef, order: CFArrayRef) -> Boolean;

    pub fn SCNetworkReachabilityCreateWithAddress(
        allocator: CFAllocatorRef,
        address: *const libc::sockaddr,
    ) -> SCNetworkReachabilityRef;
    pub fn SCNetworkReachabilityCreateWithAddressPair(
        allocator: CFAllocatorRef,
        local_address: *const libc::sockaddr,
        remote_address: *const libc::sockaddr,
    ) -> SCNetworkReachabilityRef;
    pub fn SCNetworkReachabilityCreateWithName(
        allocator: CFAllocatorRef,
        nodename: *const c_char,
    ) -> SCNetworkReachabilityRef;
    pub fn SCNetworkReachabilityGetFlags(
        target: SCNetworkReachabilityRef,
        flags: *mut SCNetworkReachabilityFlags,
    ) -> Boolean;
    pub fn SCNetworkReachabilitySetCallback(
        target: SCNetworkReachabilityRef,
        callback: SCNetworkReachabilityCallBack,
        context: *const SCNetworkReachabilityContext,
    ) -> Boolean;
    pub fn SCNetworkReachabilityScheduleWithRunLoop(
        target: SCNetworkReachabilityRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    ) -> Boolean;
    pub fn SCNetworkReachabilityUnscheduleFromRunLoop(
        target: SCNetworkReachabilityRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    ) -> Boolean;

    pub fn CNSetSupportedSSIDs(ssid_array: CFArrayRef) -> Boolean;
    pub fn CNMarkPortalOnline(interface_name: CFStringRef) -> Boolean;
    pub fn CNMarkPortalOffline(interface_name: CFStringRef) -> Boolean;
    pub fn CNCopySupportedInterfaces() -> CFArrayRef;
}
