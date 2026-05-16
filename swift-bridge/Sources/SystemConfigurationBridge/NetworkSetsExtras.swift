import Foundation
import SystemConfiguration

@_cdecl("sc_network_set_get_type_id")
public func sc_network_set_get_type_id() -> UInt64 {
    UInt64(SCNetworkSetGetTypeID())
}

@_cdecl("sc_network_set_create")
public func sc_network_set_create(_ prefsRaw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(prefsRaw), let set = SCNetworkSetCreate(prefs.value) else {
        return nil
    }
    return retain(NetworkSetBox(set))
}

@_cdecl("sc_network_set_copy")
public func sc_network_set_copy(
    _ prefsRaw: UnsafeMutableRawPointer?,
    _ setID: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(prefsRaw), let setID = decodeCString(setID),
          let set = SCNetworkSetCopy(prefs.value, setID as CFString)
    else {
        return nil
    }
    return retain(NetworkSetBox(set))
}
