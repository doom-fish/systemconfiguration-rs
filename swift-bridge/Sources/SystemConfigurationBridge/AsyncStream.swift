import Foundation
import SystemConfiguration

public typealias AsyncEventCallback = @convention(c) (
    Int32, UnsafeMutableRawPointer?, UnsafeMutableRawPointer
) -> Void

private final class RunLoopCapture {
    var runLoop: CFRunLoop?
}

private final class DynamicStoreAsyncCallbackBox {
    let onEvent: AsyncEventCallback
    let ctx: UnsafeMutableRawPointer

    init(onEvent: @escaping AsyncEventCallback, ctx: UnsafeMutableRawPointer) {
        self.onEvent = onEvent
        self.ctx = ctx
    }
}

private final class DynamicStoreNotificationHolder {
    let store: SCDynamicStore
    let runLoop: CFRunLoop
    let source: CFRunLoopSource
    let thread: Thread
    let callbackBox: DynamicStoreAsyncCallbackBox
    /// Signalled by the run-loop thread immediately after `CFRunLoopRun()` returns,
    /// allowing `unsubscribe` to join the thread and guarantee no further callbacks
    /// can fire before the Rust side drops the sender box.
    let threadDone: DispatchSemaphore

    init(
        store: SCDynamicStore,
        runLoop: CFRunLoop,
        source: CFRunLoopSource,
        thread: Thread,
        callbackBox: DynamicStoreAsyncCallbackBox,
        threadDone: DispatchSemaphore
    ) {
        self.store = store
        self.runLoop = runLoop
        self.source = source
        self.thread = thread
        self.callbackBox = callbackBox
        self.threadDone = threadDone
    }
}

private func dynamicStoreAsyncCallback(
    _ store: SCDynamicStore,
    _ changedKeys: CFArray?,
    _ info: UnsafeMutableRawPointer?
) {
    guard let info else { return }
    let box = Unmanaged<DynamicStoreAsyncCallbackBox>.fromOpaque(info).takeUnretainedValue()
    let strings = (changedKeys as NSArray?)?.compactMap { $0 as? String } ?? []
    let payload = boxStrings(strings)
    box.onEvent(0, payload, box.ctx)
}

@_cdecl("sc_dynamic_store_notification_subscribe")
public func sc_dynamic_store_notification_subscribe(
    _ name: UnsafePointer<CChar>?,
    _ keys: UnsafePointer<UnsafePointer<CChar>?>?,
    _ keyCount: Int,
    _ patterns: UnsafePointer<UnsafePointer<CChar>?>?,
    _ patternCount: Int,
    _ onEvent: AsyncEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    guard let nameStr = decodeCString(name) else { return nil }

    let callbackBox = DynamicStoreAsyncCallbackBox(onEvent: onEvent, ctx: ctx)
    var context = SCDynamicStoreContext(
        version: 0,
        info: Unmanaged.passUnretained(callbackBox).toOpaque(),
        retain: nil,
        release: nil,
        copyDescription: nil
    )

    let store = withUnsafeMutablePointer(to: &context) { ctxPtr in
        SCDynamicStoreCreate(nil, nameStr as CFString, dynamicStoreAsyncCallback, ctxPtr)
    }
    guard let store else { return nil }

    let keyArray = decodeCStringArray(keys, count: keyCount)
    let patternArray = decodeCStringArray(patterns, count: patternCount)
    guard SCDynamicStoreSetNotificationKeys(
        store,
        keyArray.isEmpty ? nil : keyArray as CFArray,
        patternArray.isEmpty ? nil : patternArray as CFArray
    ) else { return nil }

    guard let source = SCDynamicStoreCreateRunLoopSource(nil, store, 0) else { return nil }

    let semaphore = DispatchSemaphore(value: 0)
    let threadDone = DispatchSemaphore(value: 0)
    let capture = RunLoopCapture()
    let thread = Thread {
        let runLoop = CFRunLoopGetCurrent()
        capture.runLoop = runLoop
        CFRunLoopAddSource(runLoop, source, .defaultMode)
        semaphore.signal()
        CFRunLoopRun()
        // Signal after CFRunLoopRun() returns so unsubscribe can join the thread.
        threadDone.signal()
    }
    thread.name = "systemconfiguration-rs.async-dynamic-store"
    thread.start()
    semaphore.wait()

    guard let runLoop = capture.runLoop else { return nil }
    let holder = DynamicStoreNotificationHolder(
        store: store,
        runLoop: runLoop,
        source: source,
        thread: thread,
        callbackBox: callbackBox,
        threadDone: threadDone
    )
    return Unmanaged.passRetained(holder).toOpaque()
}

@_cdecl("sc_dynamic_store_notification_unsubscribe")
public func sc_dynamic_store_notification_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let holder = Unmanaged<DynamicStoreNotificationHolder>.fromOpaque(handle).takeRetainedValue()
    CFRunLoopRemoveSource(holder.runLoop, holder.source, .defaultMode)
    CFRunLoopStop(holder.runLoop)
    CFRunLoopWakeUp(holder.runLoop)
    // Join the run-loop thread: wait until CFRunLoopRun() has returned on the
    // worker thread.  This guarantees no further callbacks can access the Rust
    // sender box after this function returns, so the Rust Drop impl can safely
    // free the sender immediately after calling unsubscribe.
    holder.threadDone.wait()
}

private final class ReachabilityAsyncCallbackBox {
    let onEvent: AsyncEventCallback
    let ctx: UnsafeMutableRawPointer

    init(onEvent: @escaping AsyncEventCallback, ctx: UnsafeMutableRawPointer) {
        self.onEvent = onEvent
        self.ctx = ctx
    }
}

private final class ReachabilityNotificationHolder {
    let reachability: SCNetworkReachability
    let queue: DispatchQueue
    let callbackBox: ReachabilityAsyncCallbackBox

    init(
        reachability: SCNetworkReachability,
        queue: DispatchQueue,
        callbackBox: ReachabilityAsyncCallbackBox
    ) {
        self.reachability = reachability
        self.queue = queue
        self.callbackBox = callbackBox
    }
}

private func reachabilityAsyncCallback(
    _ reachability: SCNetworkReachability,
    _ flags: SCNetworkReachabilityFlags,
    _ info: UnsafeMutableRawPointer?
) {
    guard let info else { return }
    let box = Unmanaged<ReachabilityAsyncCallbackBox>.fromOpaque(info).takeUnretainedValue()
    box.onEvent(Int32(bitPattern: flags.rawValue), nil, box.ctx)
}

@_cdecl("sc_reachability_notification_subscribe")
public func sc_reachability_notification_subscribe(
    _ name: UnsafePointer<CChar>?,
    _ onEvent: AsyncEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    guard let nameStr = decodeCString(name) else { return nil }
    let reachability = nameStr.withCString { ptr in
        SCNetworkReachabilityCreateWithName(nil, ptr)
    }
    guard let reachability else { return nil }

    let callbackBox = ReachabilityAsyncCallbackBox(onEvent: onEvent, ctx: ctx)
    var context = SCNetworkReachabilityContext(
        version: 0,
        info: Unmanaged.passUnretained(callbackBox).toOpaque(),
        retain: nil,
        release: nil,
        copyDescription: nil
    )
    let callbackSet = withUnsafeMutablePointer(to: &context) { ctxPtr in
        SCNetworkReachabilitySetCallback(reachability, reachabilityAsyncCallback, ctxPtr)
    }
    guard callbackSet else { return nil }

    let queue = DispatchQueue(label: "systemconfiguration-rs.async-reachability", qos: .utility)
    guard SCNetworkReachabilitySetDispatchQueue(reachability, queue) else {
        SCNetworkReachabilitySetCallback(reachability, nil, nil)
        return nil
    }

    let holder = ReachabilityNotificationHolder(
        reachability: reachability,
        queue: queue,
        callbackBox: callbackBox
    )
    return Unmanaged.passRetained(holder).toOpaque()
}

@_cdecl("sc_reachability_notification_unsubscribe")
public func sc_reachability_notification_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let holder = Unmanaged<ReachabilityNotificationHolder>.fromOpaque(handle).takeRetainedValue()
    SCNetworkReachabilitySetDispatchQueue(holder.reachability, nil)
    SCNetworkReachabilitySetCallback(holder.reachability, nil, nil)
}

private final class PreferencesAsyncCallbackBox {
    let onEvent: AsyncEventCallback
    let ctx: UnsafeMutableRawPointer

    init(onEvent: @escaping AsyncEventCallback, ctx: UnsafeMutableRawPointer) {
        self.onEvent = onEvent
        self.ctx = ctx
    }
}

private final class PreferencesNotificationHolder {
    let prefs: SCPreferences
    let queue: DispatchQueue
    let callbackBox: PreferencesAsyncCallbackBox

    init(
        prefs: SCPreferences,
        queue: DispatchQueue,
        callbackBox: PreferencesAsyncCallbackBox
    ) {
        self.prefs = prefs
        self.queue = queue
        self.callbackBox = callbackBox
    }
}

private func preferencesAsyncCallback(
    _ prefs: SCPreferences,
    _ notificationType: SCPreferencesNotification,
    _ info: UnsafeMutableRawPointer?
) {
    guard let info else { return }
    let box = Unmanaged<PreferencesAsyncCallbackBox>.fromOpaque(info).takeUnretainedValue()
    box.onEvent(Int32(bitPattern: notificationType.rawValue), nil, box.ctx)
}

@_cdecl("sc_preferences_notification_subscribe")
public func sc_preferences_notification_subscribe(
    _ name: UnsafePointer<CChar>?,
    _ onEvent: AsyncEventCallback,
    _ ctx: UnsafeMutableRawPointer
) -> UnsafeMutableRawPointer? {
    guard let nameStr = decodeCString(name),
          let prefs = SCPreferencesCreate(nil, nameStr as CFString, nil)
    else { return nil }

    let callbackBox = PreferencesAsyncCallbackBox(onEvent: onEvent, ctx: ctx)
    var context = SCPreferencesContext(
        version: 0,
        info: Unmanaged.passUnretained(callbackBox).toOpaque(),
        retain: nil,
        release: nil,
        copyDescription: nil
    )
    let callbackSet = withUnsafeMutablePointer(to: &context) { ctxPtr in
        SCPreferencesSetCallback(prefs, preferencesAsyncCallback, ctxPtr)
    }
    guard callbackSet else { return nil }

    let queue = DispatchQueue(label: "systemconfiguration-rs.async-preferences", qos: .utility)
    guard SCPreferencesSetDispatchQueue(prefs, queue) else {
        SCPreferencesSetCallback(prefs, nil, nil)
        return nil
    }

    let holder = PreferencesNotificationHolder(
        prefs: prefs,
        queue: queue,
        callbackBox: callbackBox
    )
    return Unmanaged.passRetained(holder).toOpaque()
}

@_cdecl("sc_preferences_notification_unsubscribe")
public func sc_preferences_notification_unsubscribe(_ handle: UnsafeMutableRawPointer) {
    let holder = Unmanaged<PreferencesNotificationHolder>.fromOpaque(handle).takeRetainedValue()
    SCPreferencesSetDispatchQueue(holder.prefs, nil)
    SCPreferencesSetCallback(holder.prefs, nil, nil)
}
