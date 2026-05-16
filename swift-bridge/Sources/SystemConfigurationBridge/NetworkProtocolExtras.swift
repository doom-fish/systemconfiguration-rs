import Foundation
import SystemConfiguration

@_cdecl("sc_network_protocol_get_type_id")
public func sc_network_protocol_get_type_id() -> UInt64 {
    UInt64(SCNetworkProtocolGetTypeID())
}
