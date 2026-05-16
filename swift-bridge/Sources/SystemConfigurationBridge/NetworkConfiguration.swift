import Foundation
import SystemConfiguration

private let interfaceTypeConstants: [String: String] = [
    "6to4": kSCNetworkInterfaceType6to4 as String,
    "Bluetooth": kSCNetworkInterfaceTypeBluetooth as String,
    "Bond": kSCNetworkInterfaceTypeBond as String,
    "Ethernet": kSCNetworkInterfaceTypeEthernet as String,
    "FireWire": kSCNetworkInterfaceTypeFireWire as String,
    "IEEE80211": kSCNetworkInterfaceTypeIEEE80211 as String,
    "IPSec": kSCNetworkInterfaceTypeIPSec as String,
    "L2TP": kSCNetworkInterfaceTypeL2TP as String,
    "Modem": kSCNetworkInterfaceTypeModem as String,
    "PPP": kSCNetworkInterfaceTypePPP as String,
    "Serial": kSCNetworkInterfaceTypeSerial as String,
    "VLAN": kSCNetworkInterfaceTypeVLAN as String,
    "WWAN": kSCNetworkInterfaceTypeWWAN as String,
    "IPv4": kSCNetworkInterfaceTypeIPv4 as String,
    "IrDA": kSCNetworkInterfaceTypeIrDA as String,
    "PPTP": kSCNetworkInterfaceTypePPTP as String,
]

private let protocolTypeConstants: [String: String] = [
    "DNS": kSCNetworkProtocolTypeDNS as String,
    "IPv4": kSCNetworkProtocolTypeIPv4 as String,
    "IPv6": kSCNetworkProtocolTypeIPv6 as String,
    "Proxies": kSCNetworkProtocolTypeProxies as String,
    "SMB": kSCNetworkProtocolTypeSMB as String,
]

@_cdecl("sc_network_configuration_copy_overview")
public func sc_network_configuration_copy_overview() -> UnsafeMutableRawPointer? {
    let prefs = SCPreferencesCreate(nil, "systemconfiguration-rs.NetworkConfiguration" as CFString, nil)
    let interfaces = (SCNetworkInterfaceCopyAll() as NSArray).map { String(describing: $0) }
    let services = prefs.flatMap { SCNetworkServiceCopyAll($0) as NSArray? } ?? []
    let sets = prefs.flatMap { SCNetworkSetCopyAll($0) as NSArray? } ?? []
    let currentSet = prefs.flatMap { SCNetworkSetCopyCurrent($0) }

    let payload: [String: Any] = [
        "interface_count": interfaces.count,
        "service_count": services.count,
        "set_count": sets.count,
        "current_set_name": currentSet.flatMap { SCNetworkSetGetName($0) as String? } as Any,
        "current_set_id": currentSet.flatMap { SCNetworkSetGetSetID($0) as String? } as Any,
        "interface_types": interfaceTypeConstants,
        "protocol_types": protocolTypeConstants,
    ]
    return boxString(jsonString(from: payload))
}
