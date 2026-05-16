import Foundation
import SystemConfiguration

final class NetworkConnectionCallbackBox {
    let callback: RustNetworkConnectionCallback
    let info: UnsafeMutableRawPointer?

    init(callback: @escaping RustNetworkConnectionCallback, info: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.info = info
    }
}

final class NetworkConnectionBox {
    let value: SCNetworkConnection
    var callbackBox: NetworkConnectionCallbackBox?
    var dispatchQueue: DispatchQueue?

    init(_ value: SCNetworkConnection) {
        self.value = value
        callbackBox = nil
        dispatchQueue = nil
    }
}

public typealias RustNetworkConnectionCallback = @convention(c) (Int32, UnsafeMutableRawPointer?) -> Void

private func networkConnection(_ raw: UnsafeMutableRawPointer?) -> NetworkConnectionBox? {
    unretained(raw)
}

private func networkConnectionCallback(
    _ connection: SCNetworkConnection,
    _ status: SCNetworkConnectionStatus,
    _ info: UnsafeMutableRawPointer?
) {
    guard let info else {
        return
    }

    let callbackBox = Unmanaged<NetworkConnectionCallbackBox>.fromOpaque(info).takeUnretainedValue()
    callbackBox.callback(status.rawValue, callbackBox.info)
}

private func copyUserPreferences() -> (serviceID: String?, userOptions: NSDictionary?)? {
    var serviceID: Unmanaged<CFString>?
    var userOptions: Unmanaged<CFDictionary>?
    guard SCNetworkConnectionCopyUserPreferences(nil, &serviceID, &userOptions) else {
        return nil
    }

    return (
        serviceID: serviceID?.takeRetainedValue() as String?,
        userOptions: userOptions?.takeRetainedValue() as NSDictionary?
    )
}

@_cdecl("sc_network_connection_get_type_id")
public func sc_network_connection_get_type_id() -> UInt64 {
    UInt64(SCNetworkConnectionGetTypeID())
}

@_cdecl("sc_network_connection_create_with_service_id")
public func sc_network_connection_create_with_service_id(
    _ serviceID: UnsafePointer<CChar>?,
    _ callback: RustNetworkConnectionCallback?,
    _ info: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let serviceID = decodeCString(serviceID) else {
        return nil
    }

    let callbackBox = callback.map { NetworkConnectionCallbackBox(callback: $0, info: info) }
    var context = SCNetworkConnectionContext(
        version: 0,
        info: callbackBox.map { Unmanaged.passUnretained($0).toOpaque() },
        retain: nil,
        release: nil,
        copyDescription: nil
    )

    let connection: SCNetworkConnection?
    if callbackBox == nil {
        connection = SCNetworkConnectionCreateWithServiceID(nil, serviceID as CFString, nil, nil)
    } else {
        connection = withUnsafeMutablePointer(to: &context) { contextPtr in
            SCNetworkConnectionCreateWithServiceID(
                nil,
                serviceID as CFString,
                networkConnectionCallback,
                contextPtr
            )
        }
    }

    guard let connection else {
        return nil
    }

    let box = NetworkConnectionBox(connection)
    box.callbackBox = callbackBox
    return retain(box)
}

@_cdecl("sc_network_connection_copy_user_preferences_service_id")
public func sc_network_connection_copy_user_preferences_service_id() -> UnsafeMutableRawPointer? {
    boxString(copyUserPreferences()?.serviceID)
}

@_cdecl("sc_network_connection_copy_user_preferences_user_options")
public func sc_network_connection_copy_user_preferences_user_options() -> UnsafeMutableRawPointer? {
    boxPropertyList(copyUserPreferences()?.userOptions)
}

@_cdecl("sc_network_connection_copy_service_id")
public func sc_network_connection_copy_service_id(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkConnection(raw) else {
        return nil
    }
    return boxString(SCNetworkConnectionCopyServiceID(box.value) as String?)
}

@_cdecl("sc_network_connection_get_status")
public func sc_network_connection_get_status(_ raw: UnsafeMutableRawPointer?) -> Int32 {
    guard let box = networkConnection(raw) else {
        return -1
    }
    return SCNetworkConnectionGetStatus(box.value).rawValue
}

@_cdecl("sc_network_connection_copy_extended_status")
public func sc_network_connection_copy_extended_status(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkConnection(raw) else {
        return nil
    }
    return boxPropertyList(SCNetworkConnectionCopyExtendedStatus(box.value))
}

@_cdecl("sc_network_connection_copy_statistics")
public func sc_network_connection_copy_statistics(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkConnection(raw) else {
        return nil
    }
    return boxPropertyList(SCNetworkConnectionCopyStatistics(box.value))
}

@_cdecl("sc_network_connection_copy_user_options")
public func sc_network_connection_copy_user_options(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = networkConnection(raw) else {
        return nil
    }
    return boxPropertyList(SCNetworkConnectionCopyUserOptions(box.value))
}

@_cdecl("sc_network_connection_start")
public func sc_network_connection_start(
    _ raw: UnsafeMutableRawPointer?,
    _ userOptionsRaw: UnsafeMutableRawPointer?,
    _ linger: UInt8
) -> UInt8 {
    guard let box = networkConnection(raw) else {
        return 0
    }
    let userOptions = unretained(userOptionsRaw, as: PropertyListBox.self)?.value as? NSDictionary
    return u8(SCNetworkConnectionStart(box.value, userOptions, linger != 0))
}

@_cdecl("sc_network_connection_stop")
public func sc_network_connection_stop(_ raw: UnsafeMutableRawPointer?, _ forceDisconnect: UInt8) -> UInt8 {
    guard let box = networkConnection(raw) else {
        return 0
    }
    return u8(SCNetworkConnectionStop(box.value, forceDisconnect != 0))
}

@_cdecl("sc_network_connection_schedule_with_run_loop_current")
public func sc_network_connection_schedule_with_run_loop_current(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkConnection(raw) else {
        return 0
    }
    return u8(
        SCNetworkConnectionScheduleWithRunLoop(
            box.value,
            CFRunLoopGetCurrent(),
            CFRunLoopMode.defaultMode.rawValue as CFString
        )
    )
}

@_cdecl("sc_network_connection_unschedule_from_run_loop_current")
public func sc_network_connection_unschedule_from_run_loop_current(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkConnection(raw) else {
        return 0
    }
    return u8(
        SCNetworkConnectionUnscheduleFromRunLoop(
            box.value,
            CFRunLoopGetCurrent(),
            CFRunLoopMode.defaultMode.rawValue as CFString
        )
    )
}

@_cdecl("sc_network_connection_set_dispatch_queue_global")
public func sc_network_connection_set_dispatch_queue_global(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkConnection(raw) else {
        return 0
    }
    let queue = DispatchQueue(label: "systemconfiguration-rs.network-connection")
    let ok = SCNetworkConnectionSetDispatchQueue(box.value, queue)
    if ok {
        box.dispatchQueue = queue
    }
    return u8(ok)
}

@_cdecl("sc_network_connection_clear_dispatch_queue")
public func sc_network_connection_clear_dispatch_queue(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = networkConnection(raw) else {
        return 0
    }
    let ok = SCNetworkConnectionSetDispatchQueue(box.value, nil)
    if ok {
        box.dispatchQueue = nil
    }
    return u8(ok)
}
