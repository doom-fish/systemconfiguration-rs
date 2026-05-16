import Foundation
import SystemConfiguration

@_cdecl("sc_network_service_get_type_id")
public func sc_network_service_get_type_id() -> UInt64 {
    UInt64(SCNetworkServiceGetTypeID())
}

@_cdecl("sc_network_service_create")
public func sc_network_service_create(
    _ prefsRaw: UnsafeMutableRawPointer?,
    _ interfaceRaw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(prefsRaw), let interface = networkInterface(interfaceRaw),
          let service = SCNetworkServiceCreate(prefs.value, interface.value)
    else {
        return nil
    }
    return retain(NetworkServiceBox(service))
}

@_cdecl("sc_network_service_copy")
public func sc_network_service_copy(
    _ prefsRaw: UnsafeMutableRawPointer?,
    _ serviceID: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(prefsRaw), let serviceID = decodeCString(serviceID),
          let service = SCNetworkServiceCopy(prefs.value, serviceID as CFString)
    else {
        return nil
    }
    return retain(NetworkServiceBox(service))
}
