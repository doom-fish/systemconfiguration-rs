import Foundation
import SystemConfiguration

final class ReachabilityBox {
    let value: SCNetworkReachability
    var callback: RustReachabilityCallback?
    var callbackInfo: UnsafeMutableRawPointer?
    var scheduled: Bool

    init(_ value: SCNetworkReachability) {
        self.value = value
        callback = nil
        callbackInfo = nil
        scheduled = false
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

    let box = Unmanaged<ReachabilityBox>.fromOpaque(info).takeUnretainedValue()
    guard let callback = box.callback else {
        return
    }
    callback(flags.rawValue, box.callbackInfo)
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
    _ info: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let raw, let box = reachability(raw) else {
        return 0
    }

    guard let callback else {
        box.callback = nil
        box.callbackInfo = nil
        return u8(SCNetworkReachabilitySetCallback(box.value, nil, nil))
    }

    var context = SCNetworkReachabilityContext(
        version: 0,
        info: raw,
        retain: nil,
        release: nil,
        copyDescription: nil
    )
    box.callback = callback
    box.callbackInfo = info
    return u8(SCNetworkReachabilitySetCallback(box.value, reachabilityCallback, &context))
}

@_cdecl("sc_reachability_schedule_with_run_loop_current")
public func sc_reachability_schedule_with_run_loop_current(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = reachability(raw) else {
        return 0
    }

    let scheduled = SCNetworkReachabilityScheduleWithRunLoop(
        box.value,
        CFRunLoopGetCurrent(),
        CFRunLoopMode.defaultMode.rawValue as CFString
    )
    box.scheduled = scheduled
    return u8(scheduled)
}

@_cdecl("sc_reachability_unschedule_from_run_loop_current")
public func sc_reachability_unschedule_from_run_loop_current(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = reachability(raw) else {
        return 0
    }

    let scheduled = SCNetworkReachabilityUnscheduleFromRunLoop(
        box.value,
        CFRunLoopGetCurrent(),
        CFRunLoopMode.defaultMode.rawValue as CFString
    )
    if scheduled {
        box.scheduled = false
    }
    return u8(scheduled)
}
