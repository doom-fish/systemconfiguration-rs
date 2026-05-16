import Foundation
import Security
import SystemConfiguration

final class PreferencesCallbackBox {
    let callback: RustPreferencesCallback
    let info: UnsafeMutableRawPointer?

    init(callback: @escaping RustPreferencesCallback, info: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.info = info
    }
}

final class PreferencesBox {
    let value: SCPreferences
    var callbackBox: PreferencesCallbackBox?
    var dispatchQueue: DispatchQueue?

    init(_ value: SCPreferences) {
        self.value = value
        callbackBox = nil
        dispatchQueue = nil
    }
}

public typealias RustPreferencesCallback = @convention(c) (UInt32, UnsafeMutableRawPointer?) -> Void

func preferences(_ raw: UnsafeMutableRawPointer?) -> PreferencesBox? {
    unretained(raw)
}

private func preferencesCallback(
    _ prefs: SCPreferences,
    _ notificationType: SCPreferencesNotification,
    _ info: UnsafeMutableRawPointer?
) {
    guard let info else {
        return
    }

    let callbackBox = Unmanaged<PreferencesCallbackBox>.fromOpaque(info).takeUnretainedValue()
    callbackBox.callback(notificationType.rawValue, callbackBox.info)
}

@_cdecl("sc_preferences_get_type_id")
public func sc_preferences_get_type_id() -> UInt64 {
    UInt64(SCPreferencesGetTypeID())
}

@_cdecl("sc_preferences_create")
public func sc_preferences_create(
    _ name: UnsafePointer<CChar>?,
    _ prefsID: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let name = decodeCString(name) else {
        return nil
    }

    guard let prefs = SCPreferencesCreate(nil, name as CFString, decodeCString(prefsID) as CFString?) else {
        return nil
    }
    return retain(PreferencesBox(prefs))
}

@_cdecl("sc_preferences_create_with_authorization")
public func sc_preferences_create_with_authorization(
    _ name: UnsafePointer<CChar>?,
    _ prefsID: UnsafePointer<CChar>?,
    _ authorizationRaw: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let name = decodeCString(name) else {
        return nil
    }

    let authorization = authorizationRaw.map(OpaquePointer.init)
    guard let prefs = SCPreferencesCreateWithAuthorization(
        nil,
        name as CFString,
        decodeCString(prefsID) as CFString?,
        authorization
    ) else {
        return nil
    }
    return retain(PreferencesBox(prefs))
}

@_cdecl("sc_preferences_lock")
public func sc_preferences_lock(_ raw: UnsafeMutableRawPointer?, _ wait: UInt8) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }
    return u8(SCPreferencesLock(box.value, wait != 0))
}

@_cdecl("sc_preferences_commit_changes")
public func sc_preferences_commit_changes(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }
    return u8(SCPreferencesCommitChanges(box.value))
}

@_cdecl("sc_preferences_apply_changes")
public func sc_preferences_apply_changes(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }
    return u8(SCPreferencesApplyChanges(box.value))
}

@_cdecl("sc_preferences_unlock")
public func sc_preferences_unlock(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }
    return u8(SCPreferencesUnlock(box.value))
}

@_cdecl("sc_preferences_synchronize")
public func sc_preferences_synchronize(_ raw: UnsafeMutableRawPointer?) {
    guard let box = preferences(raw) else {
        return
    }
    SCPreferencesSynchronize(box.value)
}

@_cdecl("sc_preferences_copy_signature")
public func sc_preferences_copy_signature(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = preferences(raw), let data = SCPreferencesGetSignature(box.value) as Data? else {
        return nil
    }
    return boxString(data.map { String(format: "%02x", $0) }.joined())
}

@_cdecl("sc_preferences_copy_key_list")
public func sc_preferences_copy_key_list(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = preferences(raw) else {
        return nil
    }

    let strings = (SCPreferencesCopyKeyList(box.value) as NSArray?)?.map {
        $0 as? String ?? String(describing: $0)
    } ?? []
    return boxStrings(strings)
}

@_cdecl("sc_preferences_get_value")
public func sc_preferences_get_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let box = preferences(raw), let key = decodeCString(key) else {
        return nil
    }
    return boxPropertyList(SCPreferencesGetValue(box.value, key as CFString))
}

@_cdecl("sc_preferences_add_value")
public func sc_preferences_add_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = preferences(raw),
          let key = decodeCString(key),
          let valueBox: PropertyListBox = unretained(valueRaw)
    else {
        return 0
    }

    return u8(SCPreferencesAddValue(box.value, key as CFString, valueBox.value as CFPropertyList))
}

@_cdecl("sc_preferences_set_value")
public func sc_preferences_set_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = preferences(raw),
          let key = decodeCString(key),
          let valueBox: PropertyListBox = unretained(valueRaw)
    else {
        return 0
    }

    return u8(SCPreferencesSetValue(box.value, key as CFString, valueBox.value as CFPropertyList))
}

@_cdecl("sc_preferences_remove_value")
public func sc_preferences_remove_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = preferences(raw), let key = decodeCString(key) else {
        return 0
    }
    return u8(SCPreferencesRemoveValue(box.value, key as CFString))
}

@_cdecl("sc_preferences_set_callback")
public func sc_preferences_set_callback(
    _ raw: UnsafeMutableRawPointer?,
    _ callback: RustPreferencesCallback?,
    _ info: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }

    guard let callback else {
        let ok = SCPreferencesSetCallback(box.value, nil, nil)
        if ok {
            box.callbackBox = nil
        }
        return u8(ok)
    }

    let callbackBox = PreferencesCallbackBox(callback: callback, info: info)
    var context = SCPreferencesContext(
        version: 0,
        info: Unmanaged.passUnretained(callbackBox).toOpaque(),
        retain: nil,
        release: nil,
        copyDescription: nil
    )
    let ok = withUnsafeMutablePointer(to: &context) { contextPtr in
        SCPreferencesSetCallback(box.value, preferencesCallback, contextPtr)
    }
    if ok {
        box.callbackBox = callbackBox
    }
    return u8(ok)
}

@_cdecl("sc_preferences_schedule_with_run_loop_current")
public func sc_preferences_schedule_with_run_loop_current(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }
    return u8(
        SCPreferencesScheduleWithRunLoop(
            box.value,
            CFRunLoopGetCurrent(),
            CFRunLoopMode.defaultMode.rawValue as CFString
        )
    )
}

@_cdecl("sc_preferences_unschedule_from_run_loop_current")
public func sc_preferences_unschedule_from_run_loop_current(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }
    return u8(
        SCPreferencesUnscheduleFromRunLoop(
            box.value,
            CFRunLoopGetCurrent(),
            CFRunLoopMode.defaultMode.rawValue as CFString
        )
    )
}

@_cdecl("sc_preferences_set_dispatch_queue_global")
public func sc_preferences_set_dispatch_queue_global(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }
    let queue = DispatchQueue(label: "systemconfiguration-rs.preferences")
    let ok = SCPreferencesSetDispatchQueue(box.value, queue)
    if ok {
        box.dispatchQueue = queue
    }
    return u8(ok)
}

@_cdecl("sc_preferences_clear_dispatch_queue")
public func sc_preferences_clear_dispatch_queue(_ raw: UnsafeMutableRawPointer?) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }
    let ok = SCPreferencesSetDispatchQueue(box.value, nil)
    if ok {
        box.dispatchQueue = nil
    }
    return u8(ok)
}

@_cdecl("sc_preferences_path_create_unique_child")
public func sc_preferences_path_create_unique_child(
    _ raw: UnsafeMutableRawPointer?,
    _ prefix: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let box = preferences(raw), let prefix = decodeCString(prefix) else {
        return nil
    }
    return boxString(SCPreferencesPathCreateUniqueChild(box.value, prefix as CFString) as String?)
}

@_cdecl("sc_preferences_path_get_value")
public func sc_preferences_path_get_value(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let box = preferences(raw), let path = decodeCString(path) else {
        return nil
    }
    return boxPropertyList(SCPreferencesPathGetValue(box.value, path as CFString))
}

@_cdecl("sc_preferences_path_get_link")
public func sc_preferences_path_get_link(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let box = preferences(raw), let path = decodeCString(path) else {
        return nil
    }
    return boxString(SCPreferencesPathGetLink(box.value, path as CFString) as String?)
}

@_cdecl("sc_preferences_path_set_value")
public func sc_preferences_path_set_value(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ valueRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = preferences(raw),
          let path = decodeCString(path),
          let valueBox: PropertyListBox = unretained(valueRaw),
          let dictionary = valueBox.value as? NSDictionary
    else {
        return 0
    }

    return u8(SCPreferencesPathSetValue(box.value, path as CFString, dictionary))
}

@_cdecl("sc_preferences_path_set_link")
public func sc_preferences_path_set_link(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ link: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = preferences(raw),
          let path = decodeCString(path),
          let link = decodeCString(link)
    else {
        return 0
    }

    return u8(SCPreferencesPathSetLink(box.value, path as CFString, link as CFString))
}

@_cdecl("sc_preferences_path_remove_value")
public func sc_preferences_path_remove_value(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = preferences(raw), let path = decodeCString(path) else {
        return 0
    }
    return u8(SCPreferencesPathRemoveValue(box.value, path as CFString))
}

@_cdecl("sc_preferences_set_computer_name")
public func sc_preferences_set_computer_name(
    _ raw: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }

    let encoding = CFStringBuiltInEncodings.UTF8.rawValue
    return u8(
        SCPreferencesSetComputerName(
            box.value,
            decodeCString(name) as CFString?,
            CFStringEncoding(encoding)
        )
    )
}

@_cdecl("sc_preferences_set_local_host_name")
public func sc_preferences_set_local_host_name(
    _ raw: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = preferences(raw) else {
        return 0
    }
    return u8(SCPreferencesSetLocalHostName(box.value, decodeCString(name) as CFString?))
}
