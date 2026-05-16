import Foundation
import SystemConfiguration

private func mediaOptions(
    _ raw: UnsafeMutableRawPointer?,
    filter: Bool
) -> (current: NSDictionary?, active: NSDictionary?, available: NSArray?)? {
    guard let box = networkInterface(raw) else {
        return nil
    }

    var current: Unmanaged<CFDictionary>?
    var active: Unmanaged<CFDictionary>?
    var available: Unmanaged<CFArray>?
    guard SCNetworkInterfaceCopyMediaOptions(box.value, &current, &active, &available, filter) else {
        return nil
    }

    return (
        current: current?.takeRetainedValue() as NSDictionary?,
        active: active?.takeRetainedValue() as NSDictionary?,
        available: available?.takeRetainedValue() as NSArray?
    )
}

private func decodeNetworkInterfaceArray(
    _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
    count: Int
) -> [SCNetworkInterface] {
    guard let values, count > 0 else {
        return []
    }

    return (0..<count).compactMap { index in
        networkInterface(values[index])?.value
    }
}

final class BondStatusBox {
    let value: SCBondStatus

    init(_ value: SCBondStatus) {
        self.value = value
    }
}

private func bondStatus(_ raw: UnsafeMutableRawPointer?) -> BondStatusBox? {
    unretained(raw)
}

@_cdecl("sc_network_interface_get_type_id")
public func sc_network_interface_get_type_id() -> UInt64 {
    UInt64(SCNetworkInterfaceGetTypeID())
}

@_cdecl("sc_network_interface_copy_ipv4")
public func sc_network_interface_copy_ipv4() -> UnsafeMutableRawPointer? {
    retain(NetworkInterfaceBox(kSCNetworkInterfaceIPv4))
}

@_cdecl("sc_network_interface_copy_media_options_current")
public func sc_network_interface_copy_media_options_current(
    _ raw: UnsafeMutableRawPointer?,
    _ filter: UInt8
) -> UnsafeMutableRawPointer? {
    boxPropertyList(mediaOptions(raw, filter: filter != 0)?.current)
}

@_cdecl("sc_network_interface_copy_media_options_active")
public func sc_network_interface_copy_media_options_active(
    _ raw: UnsafeMutableRawPointer?,
    _ filter: UInt8
) -> UnsafeMutableRawPointer? {
    boxPropertyList(mediaOptions(raw, filter: filter != 0)?.active)
}

@_cdecl("sc_network_interface_copy_media_options_available")
public func sc_network_interface_copy_media_options_available(
    _ raw: UnsafeMutableRawPointer?,
    _ filter: UInt8
) -> UnsafeMutableRawPointer? {
    boxPropertyList(mediaOptions(raw, filter: filter != 0)?.available)
}

@_cdecl("sc_network_interface_copy_media_subtypes")
public func sc_network_interface_copy_media_subtypes(_ availableRaw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let valueBox: PropertyListBox = unretained(availableRaw),
          let available = valueBox.value as? NSArray,
          let subtypes = SCNetworkInterfaceCopyMediaSubTypes(available)
    else {
        return nil
    }

    let strings = (subtypes as NSArray).map { $0 as? String ?? String(describing: $0) }
    return boxStrings(strings)
}

@_cdecl("sc_network_interface_copy_media_subtype_options")
public func sc_network_interface_copy_media_subtype_options(
    _ availableRaw: UnsafeMutableRawPointer?,
    _ subtype: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let valueBox: PropertyListBox = unretained(availableRaw),
          let available = valueBox.value as? NSArray,
          let subtype = decodeCString(subtype),
          let options = SCNetworkInterfaceCopyMediaSubTypeOptions(available, subtype as CFString)
    else {
        return nil
    }

    let payload = (options as NSArray).map { option in
        (option as? NSArray)?.map { $0 as? String ?? String(describing: $0) } ?? [String(describing: option)]
    }
    return boxString(jsonString(from: payload))
}

@_cdecl("sc_network_interface_set_configuration")
public func sc_network_interface_set_configuration(
    _ raw: UnsafeMutableRawPointer?,
    _ configRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkInterface(raw) else {
        return 0
    }
    let configuration = (unretained(configRaw, as: PropertyListBox.self)?.value as? NSDictionary)
    return u8(SCNetworkInterfaceSetConfiguration(box.value, configuration))
}

@_cdecl("sc_network_interface_set_extended_configuration")
public func sc_network_interface_set_extended_configuration(
    _ raw: UnsafeMutableRawPointer?,
    _ extendedType: UnsafePointer<CChar>?,
    _ configRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkInterface(raw), let extendedType = decodeCString(extendedType) else {
        return 0
    }
    let configuration = (unretained(configRaw, as: PropertyListBox.self)?.value as? NSDictionary)
    return u8(SCNetworkInterfaceSetExtendedConfiguration(box.value, extendedType as CFString, configuration))
}

@_cdecl("sc_network_interface_set_media_options")
public func sc_network_interface_set_media_options(
    _ raw: UnsafeMutableRawPointer?,
    _ subtype: UnsafePointer<CChar>?,
    _ options: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int
) -> UInt8 {
    guard let box = networkInterface(raw) else {
        return 0
    }

    let optionValues = decodeCStringArray(options, count: count)
    return u8(
        SCNetworkInterfaceSetMediaOptions(
            box.value,
            decodeCString(subtype) as CFString?,
            optionValues.isEmpty ? nil : optionValues as CFArray
        )
    )
}

@_cdecl("sc_network_interface_set_mtu")
public func sc_network_interface_set_mtu(_ raw: UnsafeMutableRawPointer?, _ mtu: Int32) -> UInt8 {
    guard let box = networkInterface(raw) else {
        return 0
    }
    return u8(SCNetworkInterfaceSetMTU(box.value, mtu))
}

@_cdecl("sc_bond_interface_copy_all")
public func sc_bond_interface_copy_all(_ prefsRaw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(prefsRaw) else {
        return nil
    }
    return interfaceArray(SCBondInterfaceCopyAll(prefs.value) as NSArray?)
}

@_cdecl("sc_bond_interface_copy_available_member_interfaces")
public func sc_bond_interface_copy_available_member_interfaces(
    _ prefsRaw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(prefsRaw) else {
        return nil
    }
    return interfaceArray(SCBondInterfaceCopyAvailableMemberInterfaces(prefs.value) as NSArray?)
}

@_cdecl("sc_bond_interface_create")
public func sc_bond_interface_create(_ prefsRaw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(prefsRaw), let bond = SCBondInterfaceCreate(prefs.value) else {
        return nil
    }
    return retain(NetworkInterfaceBox(bond))
}

@_cdecl("sc_bond_interface_remove")
public func sc_bond_interface_remove(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkInterface(raw) else {
        return 0
    }
    return u8(SCBondInterfaceRemove(box.value))
}

@_cdecl("sc_bond_interface_copy_member_interfaces")
public func sc_bond_interface_copy_member_interfaces(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    return interfaceArray(SCBondInterfaceGetMemberInterfaces(box.value) as NSArray?)
}

@_cdecl("sc_bond_interface_copy_options")
public func sc_bond_interface_copy_options(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    return boxPropertyList(SCBondInterfaceGetOptions(box.value))
}

@_cdecl("sc_bond_interface_set_member_interfaces")
public func sc_bond_interface_set_member_interfaces(
    _ raw: UnsafeMutableRawPointer?,
    _ members: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) -> UInt8 {
    guard let box = networkInterface(raw) else {
        return 0
    }
    let values = decodeNetworkInterfaceArray(members, count: count)
    return u8(SCBondInterfaceSetMemberInterfaces(box.value, values as CFArray))
}

@_cdecl("sc_bond_interface_set_localized_display_name")
public func sc_bond_interface_set_localized_display_name(
    _ raw: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = networkInterface(raw), let name = decodeCString(name) else {
        return 0
    }
    return u8(SCBondInterfaceSetLocalizedDisplayName(box.value, name as CFString))
}

@_cdecl("sc_bond_interface_set_options")
public func sc_bond_interface_set_options(
    _ raw: UnsafeMutableRawPointer?,
    _ optionsRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkInterface(raw),
          let options = unretained(optionsRaw, as: PropertyListBox.self)?.value as? NSDictionary
    else {
        return 0
    }
    return u8(SCBondInterfaceSetOptions(box.value, options))
}

@_cdecl("sc_bond_interface_copy_status")
public func sc_bond_interface_copy_status(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw), let status = SCBondInterfaceCopyStatus(box.value) else {
        return nil
    }
    return retain(BondStatusBox(status))
}

@_cdecl("sc_bond_status_get_type_id")
public func sc_bond_status_get_type_id() -> UInt64 {
    UInt64(SCBondStatusGetTypeID())
}

@_cdecl("sc_bond_status_copy_device_aggregation_status_key")
public func sc_bond_status_copy_device_aggregation_status_key() -> UnsafeMutableRawPointer? {
    boxString(kSCBondStatusDeviceAggregationStatus as String)
}

@_cdecl("sc_bond_status_copy_device_collecting_key")
public func sc_bond_status_copy_device_collecting_key() -> UnsafeMutableRawPointer? {
    boxString(kSCBondStatusDeviceCollecting as String)
}

@_cdecl("sc_bond_status_copy_device_distributing_key")
public func sc_bond_status_copy_device_distributing_key() -> UnsafeMutableRawPointer? {
    boxString(kSCBondStatusDeviceDistributing as String)
}

@_cdecl("sc_bond_status_copy_member_interfaces")
public func sc_bond_status_copy_member_interfaces(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = bondStatus(raw) else {
        return nil
    }
    return interfaceArray(SCBondStatusGetMemberInterfaces(box.value) as NSArray?)
}

@_cdecl("sc_bond_status_copy_interface_status")
public func sc_bond_status_copy_interface_status(
    _ raw: UnsafeMutableRawPointer?,
    _ interfaceRaw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let box = bondStatus(raw) else {
        return nil
    }
    return boxPropertyList(SCBondStatusGetInterfaceStatus(box.value, networkInterface(interfaceRaw)?.value))
}

@_cdecl("sc_vlan_interface_copy_all")
public func sc_vlan_interface_copy_all(_ prefsRaw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(prefsRaw) else {
        return nil
    }
    return interfaceArray(SCVLANInterfaceCopyAll(prefs.value) as NSArray?)
}

@_cdecl("sc_vlan_interface_copy_available_physical_interfaces")
public func sc_vlan_interface_copy_available_physical_interfaces() -> UnsafeMutableRawPointer? {
    interfaceArray(SCVLANInterfaceCopyAvailablePhysicalInterfaces() as NSArray?)
}

@_cdecl("sc_vlan_interface_create")
public func sc_vlan_interface_create(
    _ prefsRaw: UnsafeMutableRawPointer?,
    _ physicalRaw: UnsafeMutableRawPointer?,
    _ tag: Int32
) -> UnsafeMutableRawPointer? {
    guard let prefs = preferences(prefsRaw), let physical = networkInterface(physicalRaw) else {
        return nil
    }
    let tagNumber = NSNumber(value: tag)
    guard let vlan = SCVLANInterfaceCreate(prefs.value, physical.value, tagNumber) else {
        return nil
    }
    return retain(NetworkInterfaceBox(vlan))
}

@_cdecl("sc_vlan_interface_remove")
public func sc_vlan_interface_remove(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkInterface(raw) else {
        return 0
    }
    return u8(SCVLANInterfaceRemove(box.value))
}

@_cdecl("sc_vlan_interface_copy_physical_interface")
public func sc_vlan_interface_copy_physical_interface(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw), let physical = SCVLANInterfaceGetPhysicalInterface(box.value) else {
        return nil
    }
    return retain(NetworkInterfaceBox(physical))
}

@_cdecl("sc_vlan_interface_get_tag")
public func sc_vlan_interface_get_tag(
    _ raw: UnsafeMutableRawPointer?,
    _ outTag: UnsafeMutablePointer<Int32>?
) -> UInt8 {
    guard let box = networkInterface(raw), let tag = SCVLANInterfaceGetTag(box.value) as? NSNumber, let outTag else {
        return 0
    }
    outTag.pointee = tag.int32Value
    return 1
}

@_cdecl("sc_vlan_interface_copy_options")
public func sc_vlan_interface_copy_options(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkInterface(raw) else {
        return nil
    }
    return boxPropertyList(SCVLANInterfaceGetOptions(box.value))
}

@_cdecl("sc_vlan_interface_set_physical_interface_and_tag")
public func sc_vlan_interface_set_physical_interface_and_tag(
    _ raw: UnsafeMutableRawPointer?,
    _ physicalRaw: UnsafeMutableRawPointer?,
    _ tag: Int32
) -> UInt8 {
    guard let box = networkInterface(raw), let physical = networkInterface(physicalRaw) else {
        return 0
    }
    return u8(SCVLANInterfaceSetPhysicalInterfaceAndTag(box.value, physical.value, NSNumber(value: tag)))
}

@_cdecl("sc_vlan_interface_set_localized_display_name")
public func sc_vlan_interface_set_localized_display_name(
    _ raw: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = networkInterface(raw), let name = decodeCString(name) else {
        return 0
    }
    return u8(SCVLANInterfaceSetLocalizedDisplayName(box.value, name as CFString))
}

@_cdecl("sc_vlan_interface_set_options")
public func sc_vlan_interface_set_options(
    _ raw: UnsafeMutableRawPointer?,
    _ optionsRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkInterface(raw),
          let options = unretained(optionsRaw, as: PropertyListBox.self)?.value as? NSDictionary
    else {
        return 0
    }
    return u8(SCVLANInterfaceSetOptions(box.value, options))
}
