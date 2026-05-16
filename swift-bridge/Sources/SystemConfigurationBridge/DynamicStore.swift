import Foundation
import SystemConfiguration
import SystemConfiguration.SCDynamicStoreCopyDHCPInfo

final class DynamicStoreCallbackBox {
    let callback: RustDynamicStoreCallback
    let info: UnsafeMutableRawPointer?

    init(callback: @escaping RustDynamicStoreCallback, info: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.info = info
    }
}

final class DynamicStoreBox {
    let value: SCDynamicStore
    var callbackBox: DynamicStoreCallbackBox?
    var dispatchQueue: DispatchQueue?

    init(_ value: SCDynamicStore) {
        self.value = value
        callbackBox = nil
        dispatchQueue = nil
    }
}

public typealias RustDynamicStoreCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Void

func dynamicStore(_ raw: UnsafeMutableRawPointer?) -> DynamicStoreBox? {
    unretained(raw)
}

func dynamicStoreCallback(
    _ store: SCDynamicStore,
    _ changedKeys: CFArray?,
    _ info: UnsafeMutableRawPointer?
) {
    guard let info else {
        return
    }

    let callbackBox = Unmanaged<DynamicStoreCallbackBox>.fromOpaque(info).takeUnretainedValue()
    let strings = (changedKeys as NSArray?)?.map { $0 as? String ?? String(describing: $0) } ?? []
    callbackBox.callback(boxStrings(strings), callbackBox.info)
}

@_cdecl("sc_dynamic_store_create")
public func sc_dynamic_store_create(
    _ name: UnsafePointer<CChar>?,
    _ useSessionKeys: UInt8
) -> UnsafeMutableRawPointer? {
    guard let name = decodeCString(name) else {
        return nil
    }

    let store: SCDynamicStore?
    if useSessionKeys != 0 {
        let options: NSDictionary = [kSCDynamicStoreUseSessionKeys as String: true]
        store = SCDynamicStoreCreateWithOptions(nil, name as CFString, options, nil, nil)
    } else {
        store = SCDynamicStoreCreate(nil, name as CFString, nil, nil)
    }

    guard let store else {
        return nil
    }
    return retain(DynamicStoreBox(store))
}

@_cdecl("sc_dynamic_store_copy_key_list")
public func sc_dynamic_store_copy_key_list(
    _ raw: UnsafeMutableRawPointer?,
    _ pattern: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let box = dynamicStore(raw), let pattern = decodeCString(pattern) else {
        return nil
    }

    let array = SCDynamicStoreCopyKeyList(box.value, pattern as CFString)
    let strings = (array as NSArray?)?.map { $0 as? String ?? String(describing: $0) } ?? []
    return boxStrings(strings)
}

@_cdecl("sc_dynamic_store_copy_value")
public func sc_dynamic_store_copy_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let box = dynamicStore(raw), let key = decodeCString(key) else {
        return nil
    }
    return boxPropertyList(SCDynamicStoreCopyValue(box.value, key as CFString))
}

@_cdecl("sc_dynamic_store_copy_multiple")
public func sc_dynamic_store_copy_multiple(
    _ raw: UnsafeMutableRawPointer?,
    _ keys: UnsafePointer<UnsafePointer<CChar>?>?,
    _ keyCount: Int,
    _ patterns: UnsafePointer<UnsafePointer<CChar>?>?,
    _ patternCount: Int
) -> UnsafeMutableRawPointer? {
    guard let box = dynamicStore(raw) else {
        return nil
    }

    let keysArray = decodeCStringArray(keys, count: keyCount)
    let patternsArray = decodeCStringArray(patterns, count: patternCount)
    let value = SCDynamicStoreCopyMultiple(
        box.value,
        keysArray.isEmpty ? nil : keysArray as CFArray,
        patternsArray.isEmpty ? nil : patternsArray as CFArray
    )
    return boxPropertyList(value)
}

@_cdecl("sc_dynamic_store_add_value")
public func sc_dynamic_store_add_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = dynamicStore(raw),
          let valueBox: PropertyListBox = unretained(valueRaw),
          let key = decodeCString(key)
    else {
        return 0
    }

    return u8(SCDynamicStoreAddValue(box.value, key as CFString, valueBox.value as CFPropertyList))
}

@_cdecl("sc_dynamic_store_add_temporary_value")
public func sc_dynamic_store_add_temporary_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = dynamicStore(raw),
          let valueBox: PropertyListBox = unretained(valueRaw),
          let key = decodeCString(key)
    else {
        return 0
    }

    return u8(SCDynamicStoreAddTemporaryValue(box.value, key as CFString, valueBox.value as CFPropertyList))
}

@_cdecl("sc_dynamic_store_set_value")
public func sc_dynamic_store_set_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ valueRaw: UnsafeMutableRawPointer?
) -> UInt8 {
    guard let box = dynamicStore(raw),
          let valueBox: PropertyListBox = unretained(valueRaw),
          let key = decodeCString(key)
    else {
        return 0
    }

    return u8(SCDynamicStoreSetValue(box.value, key as CFString, valueBox.value as CFPropertyList))
}

@_cdecl("sc_dynamic_store_remove_value")
public func sc_dynamic_store_remove_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = dynamicStore(raw), let key = decodeCString(key) else {
        return 0
    }
    return u8(SCDynamicStoreRemoveValue(box.value, key as CFString))
}

@_cdecl("sc_dynamic_store_notify_value")
public func sc_dynamic_store_notify_value(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UInt8 {
    guard let box = dynamicStore(raw), let key = decodeCString(key) else {
        return 0
    }
    return u8(SCDynamicStoreNotifyValue(box.value, key as CFString))
}

@_cdecl("sc_dynamic_store_set_notification_keys")
public func sc_dynamic_store_set_notification_keys(
    _ raw: UnsafeMutableRawPointer?,
    _ keys: UnsafePointer<UnsafePointer<CChar>?>?,
    _ keyCount: Int,
    _ patterns: UnsafePointer<UnsafePointer<CChar>?>?,
    _ patternCount: Int
) -> UInt8 {
    guard let box = dynamicStore(raw) else {
        return 0
    }

    let keysArray = decodeCStringArray(keys, count: keyCount)
    let patternsArray = decodeCStringArray(patterns, count: patternCount)
    return u8(
        SCDynamicStoreSetNotificationKeys(
            box.value,
            keysArray.isEmpty ? nil : keysArray as CFArray,
            patternsArray.isEmpty ? nil : patternsArray as CFArray
        )
    )
}

@_cdecl("sc_dynamic_store_copy_notified_keys")
public func sc_dynamic_store_copy_notified_keys(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = dynamicStore(raw) else {
        return nil
    }

    let array = SCDynamicStoreCopyNotifiedKeys(box.value)
    let strings = (array as NSArray?)?.map { $0 as? String ?? String(describing: $0) } ?? []
    return boxStrings(strings)
}

@_cdecl("sc_dynamic_store_copy_computer_name")
public func sc_dynamic_store_copy_computer_name(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let store = dynamicStore(raw)?.value
    return boxString(SCDynamicStoreCopyComputerName(store, nil) as String?)
}

@_cdecl("sc_dynamic_store_copy_local_host_name")
public func sc_dynamic_store_copy_local_host_name(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let store = dynamicStore(raw)?.value
    return boxString(SCDynamicStoreCopyLocalHostName(store) as String?)
}

@_cdecl("sc_dynamic_store_copy_location")
public func sc_dynamic_store_copy_location(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let store = dynamicStore(raw)?.value
    return boxString(SCDynamicStoreCopyLocation(store) as String?)
}

@_cdecl("sc_dynamic_store_copy_proxies")
public func sc_dynamic_store_copy_proxies(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    let store = dynamicStore(raw)?.value
    return boxPropertyList(SCDynamicStoreCopyProxies(store))
}

@_cdecl("sc_dynamic_store_copy_dhcp_info")
public func sc_dynamic_store_copy_dhcp_info(
    _ raw: UnsafeMutableRawPointer?,
    _ serviceID: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    let store = dynamicStore(raw)?.value
    let serviceID = decodeCString(serviceID)
    return boxPropertyList(SCDynamicStoreCopyDHCPInfo(store, serviceID as CFString?))
}

@_cdecl("sc_dynamic_store_key_create_network_global_entity")
public func sc_dynamic_store_key_create_network_global_entity(
    _ domain: UnsafePointer<CChar>?,
    _ entity: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let domain = decodeCString(domain), let entity = decodeCString(entity) else {
        return nil
    }
    return boxString(
        SCDynamicStoreKeyCreateNetworkGlobalEntity(nil, domain as CFString, entity as CFString) as String?
    )
}

@_cdecl("sc_dynamic_store_key_create_network_interface")
public func sc_dynamic_store_key_create_network_interface(
    _ domain: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let domain = decodeCString(domain) else {
        return nil
    }
    return boxString(SCDynamicStoreKeyCreateNetworkInterface(nil, domain as CFString) as String?)
}

@_cdecl("sc_dynamic_store_key_create_network_interface_entity")
public func sc_dynamic_store_key_create_network_interface_entity(
    _ domain: UnsafePointer<CChar>?,
    _ interfaceName: UnsafePointer<CChar>?,
    _ entity: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let domain = decodeCString(domain), let interfaceName = decodeCString(interfaceName) else {
        return nil
    }

    return boxString(
        SCDynamicStoreKeyCreateNetworkInterfaceEntity(
            nil,
            domain as CFString,
            interfaceName as CFString,
            decodeCString(entity) as CFString?
        ) as String?
    )
}

@_cdecl("sc_dynamic_store_key_create_network_service_entity")
public func sc_dynamic_store_key_create_network_service_entity(
    _ domain: UnsafePointer<CChar>?,
    _ serviceID: UnsafePointer<CChar>?,
    _ entity: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let domain = decodeCString(domain), let serviceID = decodeCString(serviceID) else {
        return nil
    }

    return boxString(
        SCDynamicStoreKeyCreateNetworkServiceEntity(
            nil,
            domain as CFString,
            serviceID as CFString,
            decodeCString(entity) as CFString?
        ) as String?
    )
}

@_cdecl("sc_dynamic_store_key_create_computer_name")
public func sc_dynamic_store_key_create_computer_name() -> UnsafeMutableRawPointer? {
    boxString(SCDynamicStoreKeyCreateComputerName(nil) as String?)
}

@_cdecl("sc_dynamic_store_key_create_console_user")
public func sc_dynamic_store_key_create_console_user() -> UnsafeMutableRawPointer? {
    boxString(SCDynamicStoreKeyCreateConsoleUser(nil) as String?)
}

@_cdecl("sc_dynamic_store_key_create_host_names")
public func sc_dynamic_store_key_create_host_names() -> UnsafeMutableRawPointer? {
    boxString(SCDynamicStoreKeyCreateHostNames(nil) as String?)
}

@_cdecl("sc_dynamic_store_key_create_location")
public func sc_dynamic_store_key_create_location() -> UnsafeMutableRawPointer? {
    boxString(SCDynamicStoreKeyCreateLocation(nil) as String?)
}

@_cdecl("sc_dynamic_store_key_create_proxies")
public func sc_dynamic_store_key_create_proxies() -> UnsafeMutableRawPointer? {
    boxString(SCDynamicStoreKeyCreateProxies(nil) as String?)
}
