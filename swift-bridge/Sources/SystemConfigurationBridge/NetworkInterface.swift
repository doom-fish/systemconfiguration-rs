import Foundation
import SystemConfiguration

final class NetworkInterfaceBox {
    let value: SCNetworkInterface

    init(_ value: SCNetworkInterface) {
        self.value = value
    }
}

func networkInterface(_ raw: UnsafeMutableRawPointer?) -> NetworkInterfaceBox? {
    unretained(raw)
}

func interfaceArray(_ array: NSArray?) -> UnsafeMutableRawPointer {
    let boxes = (array ?? []).map { NetworkInterfaceBox($0 as! SCNetworkInterface) }
    return boxArray(boxes)
}

@_cdecl("sc_network_interface_copy_all")
public func sc_network_interface_copy_all() -> UnsafeMutableRawPointer? {
    interfaceArray(SCNetworkInterfaceCopyAll() as NSArray)
}

@_cdecl("sc_network_interface_copy_supported_interface_types")
public func sc_network_interface_copy_supported_interface_types(
    _ raw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    let strings = (SCNetworkInterfaceGetSupportedInterfaceTypes(box.value) as NSArray?)?.map {
        $0 as? String ?? String(describing: $0)
    } ?? []
    return boxStrings(strings)
}

@_cdecl("sc_network_interface_copy_supported_protocol_types")
public func sc_network_interface_copy_supported_protocol_types(
    _ raw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    let strings = (SCNetworkInterfaceGetSupportedProtocolTypes(box.value) as NSArray?)?.map {
        $0 as? String ?? String(describing: $0)
    } ?? []
    return boxStrings(strings)
}

@_cdecl("sc_network_interface_create_with_interface")
public func sc_network_interface_create_with_interface(
    _ raw: UnsafeMutableRawPointer?,
    _ interfaceType: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw), let interfaceType = decodeCString(interfaceType),
          let created = SCNetworkInterfaceCreateWithInterface(box.value, interfaceType as CFString)
    else {
        return nil
    }
    return retain(NetworkInterfaceBox(created))
}

@_cdecl("sc_network_interface_copy_bsd_name")
public func sc_network_interface_copy_bsd_name(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    return boxString(SCNetworkInterfaceGetBSDName(box.value) as String?)
}

@_cdecl("sc_network_interface_copy_configuration")
public func sc_network_interface_copy_configuration(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    return boxPropertyList(SCNetworkInterfaceGetConfiguration(box.value))
}

@_cdecl("sc_network_interface_copy_extended_configuration")
public func sc_network_interface_copy_extended_configuration(
    _ raw: UnsafeMutableRawPointer?,
    _ extendedType: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw), let extendedType = decodeCString(extendedType) else {
        return nil
    }
    return boxPropertyList(SCNetworkInterfaceGetExtendedConfiguration(box.value, extendedType as CFString))
}

@_cdecl("sc_network_interface_copy_hardware_address")
public func sc_network_interface_copy_hardware_address(
    _ raw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    return boxString(SCNetworkInterfaceGetHardwareAddressString(box.value) as String?)
}

@_cdecl("sc_network_interface_copy_underlying_interface")
public func sc_network_interface_copy_underlying_interface(
    _ raw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw), let interface = SCNetworkInterfaceGetInterface(box.value) else {
        return nil
    }
    return retain(NetworkInterfaceBox(interface))
}

@_cdecl("sc_network_interface_copy_interface_type")
public func sc_network_interface_copy_interface_type(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    return boxString(SCNetworkInterfaceGetInterfaceType(box.value) as String?)
}

@_cdecl("sc_network_interface_copy_localized_display_name")
public func sc_network_interface_copy_localized_display_name(
    _ raw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    return boxString(SCNetworkInterfaceGetLocalizedDisplayName(box.value) as String?)
}

@_cdecl("sc_network_interface_copy_mtu_info")
public func sc_network_interface_copy_mtu_info(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }

    var current: Int32 = 0
    var minimum: Int32 = 0
    var maximum: Int32 = 0
    guard SCNetworkInterfaceCopyMTU(box.value, &current, &minimum, &maximum) else {
        return nil
    }

    let payload: [String: Int32] = [
        "current": current,
        "minimum": minimum,
        "maximum": maximum,
    ]
    return boxString(jsonString(from: payload))
}

@_cdecl("sc_network_interface_force_configuration_refresh")
public func sc_network_interface_force_configuration_refresh(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkInterface(raw) else {
        return 0
    }
    return u8(SCNetworkInterfaceForceConfigurationRefresh(box.value))
}
