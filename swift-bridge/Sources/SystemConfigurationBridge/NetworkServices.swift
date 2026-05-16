import Foundation
import SystemConfiguration

final class NetworkServiceBox {
    let value: SCNetworkService

    init(_ value: SCNetworkService) {
        self.value = value
    }
}

func networkService(_ raw: UnsafeMutableRawPointer?) -> NetworkServiceBox? {
    unretained(raw)
}

func serviceArray(_ array: NSArray?) -> UnsafeMutableRawPointer {
    let boxes = (array ?? []).map { NetworkServiceBox($0 as! SCNetworkService) }
    return boxArray(boxes)
}

@_cdecl("sc_network_service_copy_all")
public func sc_network_service_copy_all(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(raw) else {
        return nil
    }
    return serviceArray(SCNetworkServiceCopyAll(prefs.value) as NSArray?)
}

@_cdecl("sc_network_service_copy_protocols")
public func sc_network_service_copy_protocols(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkService(raw) else {
        return nil
    }
    return protocolArray(SCNetworkServiceCopyProtocols(box.value) as NSArray?)
}

@_cdecl("sc_network_service_copy_protocol")
public func sc_network_service_copy_protocol(
    _ raw: UnsafeMutableRawPointer?,
    _ protocolType: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let box = networkService(raw),
          let protocolType = decodeCString(protocolType),
          let protocolValue = SCNetworkServiceCopyProtocol(box.value, protocolType as CFString)
    else {
        return nil
    }
    return retain(NetworkProtocolBox(protocolValue))
}

@_cdecl("sc_network_service_add_protocol_type")
public func sc_network_service_add_protocol_type(
    _ raw: UnsafeMutableRawPointer?,
    _ protocolType: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = networkService(raw), let protocolType = decodeCString(protocolType) else {
        return 0
    }
    return u8(SCNetworkServiceAddProtocolType(box.value, protocolType as CFString))
}

@_cdecl("sc_network_service_establish_default_configuration")
public func sc_network_service_establish_default_configuration(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkService(raw) else {
        return 0
    }
    return u8(SCNetworkServiceEstablishDefaultConfiguration(box.value))
}

@_cdecl("sc_network_service_get_enabled")
public func sc_network_service_get_enabled(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkService(raw) else {
        return 0
    }
    return u8(SCNetworkServiceGetEnabled(box.value))
}

@_cdecl("sc_network_service_copy_interface")
public func sc_network_service_copy_interface(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkService(raw), let interface = SCNetworkServiceGetInterface(box.value) else {
        return nil
    }
    return retain(NetworkInterfaceBox(interface))
}

@_cdecl("sc_network_service_copy_name")
public func sc_network_service_copy_name(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkService(raw) else {
        return nil
    }
    return boxString(SCNetworkServiceGetName(box.value) as String?)
}

@_cdecl("sc_network_service_copy_service_id")
public func sc_network_service_copy_service_id(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkService(raw) else {
        return nil
    }
    return boxString(SCNetworkServiceGetServiceID(box.value) as String?)
}

@_cdecl("sc_network_service_remove")
public func sc_network_service_remove(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkService(raw) else {
        return 0
    }
    return u8(SCNetworkServiceRemove(box.value))
}

@_cdecl("sc_network_service_remove_protocol_type")
public func sc_network_service_remove_protocol_type(
    _ raw: UnsafeMutableRawPointer?,
    _ protocolType: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = networkService(raw), let protocolType = decodeCString(protocolType) else {
        return 0
    }
    return u8(SCNetworkServiceRemoveProtocolType(box.value, protocolType as CFString))
}

@_cdecl("sc_network_service_set_enabled")
public func sc_network_service_set_enabled(_ raw: UnsafeMutableRawPointer?, _ enabled: UInt8) -> UInt8 {
    guard let box = networkService(raw) else {
        return 0
    }
    return u8(SCNetworkServiceSetEnabled(box.value, enabled != 0))
}

@_cdecl("sc_network_service_set_name")
public func sc_network_service_set_name(
    _ raw: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = networkService(raw) else {
        return 0
    }
    return u8(SCNetworkServiceSetName(box.value, decodeCString(name) as CFString?))
}
