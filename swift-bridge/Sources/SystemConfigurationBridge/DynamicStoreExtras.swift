import Foundation
import SystemConfiguration
import SystemConfiguration.SCDynamicStoreCopyDHCPInfo

final class RunLoopSourceBox {
    let value: CFRunLoopSource

    init(_ value: CFRunLoopSource) {
        self.value = value
    }
}

private func runLoopSource(_ raw: UnsafeMutableRawPointer?) -> RunLoopSourceBox? {
    unretained(raw)
}

private func dynamicStoreKeyCreate(format: String, arguments: [String]) -> String? {
    let values = arguments.map { $0 as NSString }
    return withVaList(values) { pointer in
        CFStringCreateWithFormatAndArguments(nil, nil, format as CFString, pointer) as String?
    }
}

@_cdecl("sc_dynamic_store_get_type_id")
public func sc_dynamic_store_get_type_id() -> UInt64 {
    UInt64(SCDynamicStoreGetTypeID())
}

@_cdecl("sc_dynamic_store_create_with_options")
public func sc_dynamic_store_create_with_options(
    _ name: UnsafePointer<CChar>?,
    _ optionsRaw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let name = decodeCString(name),
          let options = unretained(optionsRaw, as: PropertyListBox.self)?.value as? NSDictionary
    else {
        return nil
    }

    guard let store = SCDynamicStoreCreateWithOptions(nil, name as CFString, options, nil, nil) else {
        return nil
    }
    return retain(DynamicStoreBox(store))
}

@_cdecl("sc_dynamic_store_create_with_callback")
public func sc_dynamic_store_create_with_callback(
    _ name: UnsafePointer<CChar>?,
    _ useSessionKeys: UInt8,
    _ callback: RustDynamicStoreCallback?,
    _ info: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let name = decodeCString(name), let callback else {
        return nil
    }

    let callbackBox = DynamicStoreCallbackBox(callback: callback, info: info)
    var context = SCDynamicStoreContext(
        version: 0,
        info: Unmanaged.passUnretained(callbackBox).toOpaque(),
        retain: nil,
        release: nil,
        copyDescription: nil
    )

    let store: SCDynamicStore? = withUnsafeMutablePointer(to: &context) { contextPtr in
        if useSessionKeys != 0 {
            let options: NSDictionary = [kSCDynamicStoreUseSessionKeys as String: true]
            return SCDynamicStoreCreateWithOptions(nil, name as CFString, options, dynamicStoreCallback, contextPtr)
        }
        return SCDynamicStoreCreate(nil, name as CFString, dynamicStoreCallback, contextPtr)
    }

    guard let store else {
        return nil
    }

    let box = DynamicStoreBox(store)
    box.callbackBox = callbackBox
    return retain(box)
}

@_cdecl("sc_dynamic_store_create_with_options_and_callback")
public func sc_dynamic_store_create_with_options_and_callback(
    _ name: UnsafePointer<CChar>?,
    _ optionsRaw: UnsafeMutableRawPointer?,
    _ callback: RustDynamicStoreCallback?,
    _ info: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let name = decodeCString(name),
          let callback,
          let options = unretained(optionsRaw, as: PropertyListBox.self)?.value as? NSDictionary
    else {
        return nil
    }

    let callbackBox = DynamicStoreCallbackBox(callback: callback, info: info)
    var context = SCDynamicStoreContext(
        version: 0,
        info: Unmanaged.passUnretained(callbackBox).toOpaque(),
        retain: nil,
        release: nil,
        copyDescription: nil
    )

    let store = withUnsafeMutablePointer(to: &context) { contextPtr in
        SCDynamicStoreCreateWithOptions(nil, name as CFString, options, dynamicStoreCallback, contextPtr)
    }
    guard let store else {
        return nil
    }

    let box = DynamicStoreBox(store)
    box.callbackBox = callbackBox
    return retain(box)
}

@_cdecl("sc_dynamic_store_set_multiple")
public func sc_dynamic_store_set_multiple(
    _ raw: UnsafeMutableRawPointer?,
    _ keysToSetRaw: UnsafeMutableRawPointer?,
    _ keysToRemove: UnsafePointer<UnsafePointer<CChar>?>?,
    _ removeCount: Int,
    _ keysToNotify: UnsafePointer<UnsafePointer<CChar>?>?,
    _ notifyCount: Int
) -> UInt8 {
    guard let box = dynamicStore(raw) else {
        return 0
    }

    let keysToSet = unretained(keysToSetRaw, as: PropertyListBox.self)?.value as? NSDictionary
    let removed = decodeCStringArray(keysToRemove, count: removeCount)
    let notified = decodeCStringArray(keysToNotify, count: notifyCount)
    return u8(
        SCDynamicStoreSetMultiple(
            box.value,
            keysToSet,
            removed.isEmpty ? nil : removed as CFArray,
            notified.isEmpty ? nil : notified as CFArray
        )
    )
}

@_cdecl("sc_dynamic_store_create_run_loop_source")
public func sc_dynamic_store_create_run_loop_source(
    _ raw: UnsafeMutableRawPointer?,
    _ order: Int
) -> UnsafeMutableRawPointer? {
    guard let box = dynamicStore(raw), let source = SCDynamicStoreCreateRunLoopSource(nil, box.value, order) else {
        return nil
    }
    return retain(RunLoopSourceBox(source))
}

@_cdecl("sc_run_loop_source_schedule_current_default_mode")
public func sc_run_loop_source_schedule_current_default_mode(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let source = runLoopSource(raw) else {
        return 0
    }
    CFRunLoopAddSource(CFRunLoopGetCurrent(), source.value, .defaultMode)
    return 1
}

@_cdecl("sc_run_loop_source_unschedule_current_default_mode")
public func sc_run_loop_source_unschedule_current_default_mode(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let source = runLoopSource(raw) else {
        return 0
    }
    CFRunLoopRemoveSource(CFRunLoopGetCurrent(), source.value, .defaultMode)
    return 1
}

@_cdecl("sc_dynamic_store_set_dispatch_queue_global")
public func sc_dynamic_store_set_dispatch_queue_global(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = dynamicStore(raw) else {
        return 0
    }
    let queue = DispatchQueue(label: "systemconfiguration-rs.dynamic-store")
    let ok = SCDynamicStoreSetDispatchQueue(box.value, queue)
    if ok {
        box.dispatchQueue = queue
    }
    return u8(ok)
}

@_cdecl("sc_dynamic_store_clear_dispatch_queue")
public func sc_dynamic_store_clear_dispatch_queue(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = dynamicStore(raw) else {
        return 0
    }
    let ok = SCDynamicStoreSetDispatchQueue(box.value, nil)
    if ok {
        box.dispatchQueue = nil
    }
    return u8(ok)
}

@_cdecl("sc_dhcp_info_copy_option_data")
public func sc_dhcp_info_copy_option_data(
    _ infoRaw: UnsafeMutableRawPointer?,
    _ code: UInt8
) -> UnsafeMutableRawPointer? {
    guard let info = unretained(infoRaw, as: PropertyListBox.self)?.value as? NSDictionary else {
        return nil
    }
    return boxPropertyList(DHCPInfoGetOptionData(info, code))
}

@_cdecl("sc_dhcp_info_copy_lease_start_time")
public func sc_dhcp_info_copy_lease_start_time(_ infoRaw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let info = unretained(infoRaw, as: PropertyListBox.self)?.value as? NSDictionary else {
        return nil
    }
    return boxPropertyList(DHCPInfoGetLeaseStartTime(info))
}

@_cdecl("sc_dhcp_info_copy_lease_expiration_time")
public func sc_dhcp_info_copy_lease_expiration_time(_ infoRaw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let info = unretained(infoRaw, as: PropertyListBox.self)?.value as? NSDictionary else {
        return nil
    }
    return boxPropertyList(DHCPInfoGetLeaseExpirationTime(info))
}

@_cdecl("sc_dynamic_store_key_create")
public func sc_dynamic_store_key_create(
    _ format: UnsafePointer<CChar>?,
    _ arguments: UnsafePointer<UnsafePointer<CChar>?>?,
    _ count: Int
) -> UnsafeMutableRawPointer? {
    guard let format = decodeCString(format) else {
        return nil
    }
    return boxString(dynamicStoreKeyCreate(format: format, arguments: decodeCStringArray(arguments, count: count)))
}
