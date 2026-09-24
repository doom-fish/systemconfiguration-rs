import Foundation
import SystemConfiguration

final class NetworkConnectionCallbackBox: RustCallbackContext {
    let callback: RustNetworkConnectionCallback

    init(
        callback: @escaping RustNetworkConnectionCallback,
        info: UnsafeMutableRawPointer?,
        retainInfo: RustContextCallback?,
        releaseInfo: RustContextCallback?
    ) {
        self.callback = callback
        super.init(info: info, retainInfo: retainInfo, releaseInfo: releaseInfo)
    }
}

final class NetworkConnectionBox {
    let value: SCNetworkConnection
    var dispatchQueue: DispatchQueue?
    var schedules: [RunLoopSchedule]

    init(_ value: SCNetworkConnection) {
        self.value = value
        dispatchQueue = nil
        schedules = []
    }

    deinit {
        for schedule in schedules {
            SCNetworkConnectionUnscheduleFromRunLoop(value, schedule.runLoop, schedule.mode)
        }
        if dispatchQueue != nil {
            SCNetworkConnectionSetDispatchQueue(value, nil)
        }
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
    _ info: UnsafeMutableRawPointer?,
    _ retainInfo: RustContextCallback?,
    _ releaseInfo: RustContextCallback?
) -> UnsafeMutableRawPointer? {
    guard let serviceID = decodeCString(serviceID) else {
        return nil
    }

    let connection: SCNetworkConnection?
    if let callback {
        let callbackBox = NetworkConnectionCallbackBox(
            callback: callback,
            info: info,
            retainInfo: retainInfo,
            releaseInfo: releaseInfo
        )
        var context = SCNetworkConnectionContext(
            version: 0,
            info: Unmanaged.passUnretained(callbackBox).toOpaque(),
            retain: scContextRetain,
            release: scContextRelease,
            copyDescription: nil
        )
        connection = withExtendedLifetime(callbackBox) {
            withUnsafeMutablePointer(to: &context) { contextPtr in
                SCNetworkConnectionCreateWithServiceID(
                    nil,
                    serviceID as CFString,
                    networkConnectionCallback,
                    contextPtr
                )
            }
        }
    } else {
        connection = SCNetworkConnectionCreateWithServiceID(nil, serviceID as CFString, nil, nil)
    }

    guard let connection else {
        return nil
    }
    return retain(NetworkConnectionBox(connection))
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

@_cdecl("sc_network_connection_schedule_with_run_loop")
public func sc_network_connection_schedule_with_run_loop(
    _ raw: UnsafeMutableRawPointer?,
    _ runLoopRaw: UnsafeMutableRawPointer?,
    _ modeRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkConnection(raw),
          let runLoop = runLoopArgument(runLoopRaw),
          let mode = runLoopModeArgument(modeRaw)
    else {
        return 0
    }
    let ok = SCNetworkConnectionScheduleWithRunLoop(box.value, runLoop, mode)
    if ok {
        box.schedules.append(RunLoopSchedule(runLoop: runLoop, mode: mode))
    }
    return u8(ok)
}

@_cdecl("sc_network_connection_unschedule_from_run_loop")
public func sc_network_connection_unschedule_from_run_loop(
    _ raw: UnsafeMutableRawPointer?,
    _ runLoopRaw: UnsafeMutableRawPointer?,
    _ modeRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkConnection(raw),
          let runLoop = runLoopArgument(runLoopRaw),
          let mode = runLoopModeArgument(modeRaw)
    else {
        return 0
    }
    let ok = SCNetworkConnectionUnscheduleFromRunLoop(box.value, runLoop, mode)
    if ok {
        removeSchedule(&box.schedules, runLoop, mode)
    }
    return u8(ok)
}

@_cdecl("sc_network_connection_set_dispatch_queue")
public func sc_network_connection_set_dispatch_queue(
    _ raw: UnsafeMutableRawPointer?,
    _ queueRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = networkConnection(raw), let queue = dispatchQueueArgument(queueRaw) else {
        return 0
    }
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
