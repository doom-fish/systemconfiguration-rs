import Foundation
import SystemConfiguration

final class ReachabilityCallbackBox: RustCallbackContext {
    let callback: RustReachabilityCallback

    init(
        callback: @escaping RustReachabilityCallback,
        info: UnsafeMutableRawPointer?,
        retainInfo: RustContextCallback?,
        releaseInfo: RustContextCallback?
    ) {
        self.callback = callback
        super.init(info: info, retainInfo: retainInfo, releaseInfo: releaseInfo)
    }
}

final class ReachabilityBox {
    let value: SCNetworkReachability
    var hasCallback: Bool
    var schedules: [RunLoopSchedule]
    var dispatchQueue: DispatchQueue?

    init(_ value: SCNetworkReachability) {
        self.value = value
        hasCallback = false
        schedules = []
        dispatchQueue = nil
    }

    deinit {
        for schedule in schedules {
            SCNetworkReachabilityUnscheduleFromRunLoop(value, schedule.runLoop, schedule.mode)
        }
        if dispatchQueue != nil {
            SCNetworkReachabilitySetDispatchQueue(value, nil)
        }
        if hasCallback {
            SCNetworkReachabilitySetCallback(value, nil, nil)
        }
    }
}

public typealias RustReachabilityCallback = @convention(c) (UInt32, UnsafeMutableRawPointer?) -> Void

private func reachability(_ raw: UnsafeMutableRawPointer?) -> ReachabilityBox? {
    unretained(raw)
}

private func reachabilityCallback(
    _ target: SCNetworkReachability,
    _ flags: SCNetworkReachabilityFlags,
    _ info: UnsafeMutableRawPointer?
) {
    guard let info else {
        return
    }

    let callbackBox = Unmanaged<ReachabilityCallbackBox>.fromOpaque(info).takeUnretainedValue()
    callbackBox.callback(flags.rawValue, callbackBox.info)
}

private func reachabilityFromAddressBytes(
    _ bytes: UnsafePointer<UInt8>?,
    _ count: Int
) -> SCNetworkReachability? {
    guard let bytes, count > 0 else {
        return nil
    }

    let data = Data(bytes: bytes, count: count)
    return data.withUnsafeBytes { rawBuffer in
        guard let baseAddress = rawBuffer.baseAddress?.assumingMemoryBound(to: sockaddr.self) else {
            return nil
        }
        return SCNetworkReachabilityCreateWithAddress(nil, baseAddress)
    }
}

@_cdecl("sc_reachability_get_type_id")
public func sc_reachability_get_type_id() -> UInt64 {
    UInt64(SCNetworkReachabilityGetTypeID())
}

@_cdecl("sc_reachability_create_with_name")
public func sc_reachability_create_with_name(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else {
        return nil
    }
    guard let value = SCNetworkReachabilityCreateWithName(nil, name) else {
        return nil
    }
    return retain(ReachabilityBox(value))
}

@_cdecl("sc_reachability_create_with_address")
public func sc_reachability_create_with_address(
    _ bytes: UnsafePointer<UInt8>?,
    _ count: Int
) -> UnsafeMutableRawPointer? {
    guard let value = reachabilityFromAddressBytes(bytes, count) else {
        return nil
    }
    return retain(ReachabilityBox(value))
}

@_cdecl("sc_reachability_create_with_address_pair")
public func sc_reachability_create_with_address_pair(
    _ localBytes: UnsafePointer<UInt8>?,
    _ localCount: Int,
    _ remoteBytes: UnsafePointer<UInt8>?,
    _ remoteCount: Int
) -> UnsafeMutableRawPointer? {
    let localData = localBytes.flatMap { Data(bytes: $0, count: localCount) }
    let remoteData = remoteBytes.flatMap { Data(bytes: $0, count: remoteCount) }

    let value: SCNetworkReachability? = localData?.withUnsafeBytes { localBuffer in
        let localAddress = localBuffer.baseAddress?.assumingMemoryBound(to: sockaddr.self)
        return remoteData?.withUnsafeBytes { remoteBuffer in
            let remoteAddress = remoteBuffer.baseAddress?.assumingMemoryBound(to: sockaddr.self)
            return SCNetworkReachabilityCreateWithAddressPair(nil, localAddress, remoteAddress)
        } ?? SCNetworkReachabilityCreateWithAddressPair(nil, localAddress, nil)
    } ?? remoteData?.withUnsafeBytes { remoteBuffer in
        let remoteAddress = remoteBuffer.baseAddress?.assumingMemoryBound(to: sockaddr.self)
        return SCNetworkReachabilityCreateWithAddressPair(nil, nil, remoteAddress)
    }

    guard let value else {
        return nil
    }
    return retain(ReachabilityBox(value))
}

@_cdecl("sc_reachability_get_flags")
public func sc_reachability_get_flags(
    _ raw: UnsafeMutableRawPointer?,
    _ outFlags: UnsafeMutablePointer<UInt32>?
) -> UInt8 {
    guard let box = reachability(raw), let outFlags else {
        return 0
    }

    var flags: SCNetworkReachabilityFlags = []
    guard SCNetworkReachabilityGetFlags(box.value, &flags) else {
        return 0
    }
    outFlags.pointee = flags.rawValue
    return 1
}

@_cdecl("sc_reachability_set_callback")
public func sc_reachability_set_callback(
    _ raw: UnsafeMutableRawPointer?,
    _ callback: RustReachabilityCallback?,
    _ info: UnsafeMutableRawPointer?,
    _ retainInfo: RustContextCallback?,
    _ releaseInfo: RustContextCallback?
) -> UInt8 {
    guard let box = reachability(raw) else {
        return 0
    }

    guard let callback else {
        let ok = SCNetworkReachabilitySetCallback(box.value, nil, nil)
        if ok {
            box.hasCallback = false
        }
        return u8(ok)
    }

    let callbackBox = ReachabilityCallbackBox(
        callback: callback,
        info: info,
        retainInfo: retainInfo,
        releaseInfo: releaseInfo
    )
    var context = SCNetworkReachabilityContext(
        version: 0,
        info: Unmanaged.passUnretained(callbackBox).toOpaque(),
        retain: scContextRetain,
        release: scContextRelease,
        copyDescription: nil
    )
    let ok = withExtendedLifetime(callbackBox) {
        SCNetworkReachabilitySetCallback(box.value, reachabilityCallback, &context)
    }
    if ok {
        box.hasCallback = true
    }
    return u8(ok)
}

@_cdecl("sc_reachability_schedule_with_run_loop")
public func sc_reachability_schedule_with_run_loop(
    _ raw: UnsafeMutableRawPointer?,
    _ runLoopRaw: UnsafeMutableRawPointer?,
    _ modeRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = reachability(raw),
          let runLoop = runLoopArgument(runLoopRaw),
          let mode = runLoopModeArgument(modeRaw)
    else {
        return 0
    }
    let ok = SCNetworkReachabilityScheduleWithRunLoop(box.value, runLoop, mode)
    if ok {
        box.schedules.append(RunLoopSchedule(runLoop: runLoop, mode: mode))
    }
    return u8(ok)
}

@_cdecl("sc_reachability_unschedule_from_run_loop")
public func sc_reachability_unschedule_from_run_loop(
    _ raw: UnsafeMutableRawPointer?,
    _ runLoopRaw: UnsafeMutableRawPointer?,
    _ modeRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = reachability(raw),
          let runLoop = runLoopArgument(runLoopRaw),
          let mode = runLoopModeArgument(modeRaw)
    else {
        return 0
    }
    let ok = SCNetworkReachabilityUnscheduleFromRunLoop(box.value, runLoop, mode)
    if ok {
        removeSchedule(&box.schedules, runLoop, mode)
    }
    return u8(ok)
}

@_cdecl("sc_reachability_set_dispatch_queue")
public func sc_reachability_set_dispatch_queue(
    _ raw: UnsafeMutableRawPointer?,
    _ queueRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = reachability(raw), let queue = dispatchQueueArgument(queueRaw) else {
        return 0
    }
    let ok = SCNetworkReachabilitySetDispatchQueue(box.value, queue)
    if ok {
        box.dispatchQueue = queue
    }
    return u8(ok)
}

@_cdecl("sc_reachability_set_dispatch_queue_global")
public func sc_reachability_set_dispatch_queue_global(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = reachability(raw) else {
        return 0
    }

    let queue = DispatchQueue(label: "systemconfiguration-rs.reachability")
    let ok = SCNetworkReachabilitySetDispatchQueue(box.value, queue)
    if ok {
        box.dispatchQueue = queue
    }
    return u8(ok)
}

@_cdecl("sc_reachability_clear_dispatch_queue")
public func sc_reachability_clear_dispatch_queue(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = reachability(raw) else {
        return 0
    }

    let ok = SCNetworkReachabilitySetDispatchQueue(box.value, nil)
    if ok {
        box.dispatchQueue = nil
    }
    return u8(ok)
}
