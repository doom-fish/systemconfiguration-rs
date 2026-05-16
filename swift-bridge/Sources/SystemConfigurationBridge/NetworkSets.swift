import Foundation
import SystemConfiguration

final class NetworkSetBox {
    let value: SCNetworkSet

    init(_ value: SCNetworkSet) {
        self.value = value
    }
}

func networkSet(_ raw: UnsafeMutableRawPointer?) -> NetworkSetBox? {
    unretained(raw)
}

func setArray(_ array: NSArray?) -> UnsafeMutableRawPointer {
    let boxes = (array ?? []).map { NetworkSetBox($0 as! SCNetworkSet) }
    return boxArray(boxes)
}

@_cdecl("sc_network_set_copy_all")
public func sc_network_set_copy_all(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(raw) else {
        return nil
    }
    return setArray(SCNetworkSetCopyAll(prefs.value) as NSArray?)
}

@_cdecl("sc_network_set_copy_current")
public func sc_network_set_copy_current(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(raw), let set = SCNetworkSetCopyCurrent(prefs.value) else {
        return nil
    }
    return retain(NetworkSetBox(set))
}

@_cdecl("sc_network_set_copy_services")
public func sc_network_set_copy_services(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkSet(raw) else {
        return nil
    }
    return serviceArray(SCNetworkSetCopyServices(box.value) as NSArray?)
}

@_cdecl("sc_network_set_copy_name")
public func sc_network_set_copy_name(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkSet(raw) else {
        return nil
    }
    return boxString(SCNetworkSetGetName(box.value) as String?)
}

@_cdecl("sc_network_set_copy_set_id")
public func sc_network_set_copy_set_id(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkSet(raw) else {
        return nil
    }
    return boxString(SCNetworkSetGetSetID(box.value) as String?)
}

@_cdecl("sc_network_set_copy_service_order")
public func sc_network_set_copy_service_order(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkSet(raw) else {
        return nil
    }
    let strings = (SCNetworkSetGetServiceOrder(box.value) as NSArray?)?.map {
        $0 as? String ?? String(describing: $0)
    } ?? []
    return boxStrings(strings)
}

@_cdecl("sc_network_set_contains_interface")
public func sc_network_set_contains_interface(
    _ raw: UnsafeMutableRawPointer?,
    _ interfaceRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkSet(raw), let interface = networkInterface(interfaceRaw) else {
        return 0
    }
    return u8(SCNetworkSetContainsInterface(box.value, interface.value))
}

@_cdecl("sc_network_set_add_service")
public func sc_network_set_add_service(
    _ raw: UnsafeMutableRawPointer?,
    _ serviceRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkSet(raw), let service = networkService(serviceRaw) else {
        return 0
    }
    return u8(SCNetworkSetAddService(box.value, service.value))
}

@_cdecl("sc_network_set_remove")
public func sc_network_set_remove(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkSet(raw) else {
        return 0
    }
    return u8(SCNetworkSetRemove(box.value))
}

@_cdecl("sc_network_set_remove_service")
public func sc_network_set_remove_service(
    _ raw: UnsafeMutableRawPointer?,
    _ serviceRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkSet(raw), let service = networkService(serviceRaw) else {
        return 0
    }
    return u8(SCNetworkSetRemoveService(box.value, service.value))
}

@_cdecl("sc_network_set_set_current")
public func sc_network_set_set_current(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkSet(raw) else {
        return 0
    }
    return u8(SCNetworkSetSetCurrent(box.value))
}

@_cdecl("sc_network_set_set_name")
public func sc_network_set_set_name(
    _ raw: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = networkSet(raw) else {
        return 0
    }
    return u8(SCNetworkSetSetName(box.value, decodeCString(name) as CFString?))
}

@_cdecl("sc_network_set_set_service_order")
public func sc_network_set_set_service_order(
    _ raw: UnsafeMutableRawPointer?,
    _ values: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int
) -> UInt8 {
    guard let box = networkSet(raw) else {
        return 0
    }
    let order = decodeCStringArray(values, count: count)
    return u8(SCNetworkSetSetServiceOrder(box.value, order as CFArray))
}
