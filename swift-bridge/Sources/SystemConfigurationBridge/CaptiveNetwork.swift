import Foundation
import SystemConfiguration.CaptiveNetwork

@_cdecl("sc_captive_network_copy_supported_interfaces")
public func sc_captive_network_copy_supported_interfaces() -> UnsafeMutableRawPointer? {
    let values = (CNCopySupportedInterfaces() as NSArray?)?.map { $0 as? String ?? String(describing: $0) } ?? []
    return boxStrings(values)
}

@_cdecl("sc_captive_network_set_supported_ssids")
public func sc_captive_network_set_supported_ssids(
    _ values: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int
) -> UInt8 {
    let ssids = decodeCStringArray(values, count: count)
    return u8(CNSetSupportedSSIDs(ssids as CFArray))
}

@_cdecl("sc_captive_network_mark_portal_online")
public func sc_captive_network_mark_portal_online(_ interfaceName: UnsafePointer<CChar>?) -> UInt8 {
    guard let interfaceName = decodeCString(interfaceName) else {
        return 0
    }
    return u8(CNMarkPortalOnline(interfaceName as CFString))
}

@_cdecl("sc_captive_network_mark_portal_offline")
public func sc_captive_network_mark_portal_offline(_ interfaceName: UnsafePointer<CChar>?) -> UInt8 {
    guard let interfaceName = decodeCString(interfaceName) else {
        return 0
    }
    return u8(CNMarkPortalOffline(interfaceName as CFString))
}
