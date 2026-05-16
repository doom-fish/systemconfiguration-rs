import Foundation
import SystemConfiguration

final class NetworkProtocolBox {
    let value: SCNetworkProtocol

    init(_ value: SCNetworkProtocol) {
        self.value = value
    }
}

func networkProtocol(_ raw: UnsafeMutableRawPointer?) -> NetworkProtocolBox? {
    unretained(raw)
}

func protocolArray(_ array: NSArray?) -> UnsafeMutableRawPointer {
    let boxes = (array ?? []).map { NetworkProtocolBox($0 as! SCNetworkProtocol) }
    return boxArray(boxes)
}

@_cdecl("sc_network_protocol_copy_configuration")
public func sc_network_protocol_copy_configuration(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkProtocol(raw) else {
        return nil
    }
    return boxPropertyList(SCNetworkProtocolGetConfiguration(box.value))
}

@_cdecl("sc_network_protocol_get_enabled")
public func sc_network_protocol_get_enabled(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkProtocol(raw) else {
        return 0
    }
    return u8(SCNetworkProtocolGetEnabled(box.value))
}

@_cdecl("sc_network_protocol_copy_protocol_type")
public func sc_network_protocol_copy_protocol_type(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkProtocol(raw) else {
        return nil
    }
    return boxString(SCNetworkProtocolGetProtocolType(box.value) as String?)
}

@_cdecl("sc_network_protocol_set_configuration")
public func sc_network_protocol_set_configuration(
    _ raw: UnsafeMutableRawPointer?,
    _ valueRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkProtocol(raw),
          let valueBox: PropertyListBox = unretained(valueRaw),
          let configuration = valueBox.value as? NSDictionary
    else {
        return 0
    }
    return u8(SCNetworkProtocolSetConfiguration(box.value, configuration))
}

@_cdecl("sc_network_protocol_set_enabled")
public func sc_network_protocol_set_enabled(_ raw: UnsafeMutableRawPointer?, _ enabled: UInt8) -> UInt8 {
    guard let box = networkProtocol(raw) else {
        return 0
    }
    return u8(SCNetworkProtocolSetEnabled(box.value, enabled != 0))
}
