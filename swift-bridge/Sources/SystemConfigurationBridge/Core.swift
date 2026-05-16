import Foundation
import SystemConfiguration

final class StringBox {
    let value: String

    init(_ value: String) {
        self.value = value
    }
}

final class ArrayBox {
    let values: [AnyObject]

    init(_ values: [AnyObject]) {
        self.values = values
    }
}

final class PropertyListBox {
    let value: Any

    init(_ value: Any) {
        self.value = value
    }
}

func retain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

func unretained<T: AnyObject>(_ raw: UnsafeMutableRawPointer?, as type: T.Type = T.self) -> T? {
    guard let raw else {
        return nil
    }
    return Unmanaged<T>.fromOpaque(raw).takeUnretainedValue()
}

func u8(_ value: Bool) -> UInt8 {
    value ? 1 : 0
}

func decodeCString(_ value: UnsafePointer<CChar>?) -> String? {
    guard let value else {
        return nil
    }
    return String(cString: value)
}

func decodeCStringArray(
    _ values: UnsafePointer<UnsafePointer<CChar>?>?,
    count: Int
) -> [String] {
    guard let values, count > 0 else {
        return []
    }

    return (0..<count).compactMap { index in
        guard let ptr = values[index] else {
            return nil
        }
        return String(cString: ptr)
    }
}

func boxString(_ value: String?) -> UnsafeMutableRawPointer? {
    guard let value else {
        return nil
    }
    return retain(StringBox(value))
}

func boxStrings(_ values: [String]) -> UnsafeMutableRawPointer {
    retain(ArrayBox(values.map(StringBox.init)))
}

func boxArray(_ values: [AnyObject]) -> UnsafeMutableRawPointer {
    retain(ArrayBox(values))
}

func boxPropertyList(_ value: Any?) -> UnsafeMutableRawPointer? {
    guard let value else {
        return nil
    }
    return retain(PropertyListBox(value))
}

func propertyListFromJSON(_ json: String) -> Any? {
    guard let data = json.data(using: .utf8) else {
        return nil
    }
    return try? JSONSerialization.jsonObject(with: data)
}

func propertyListDescription(_ value: Any) -> String {
    if let string = value as? String {
        return string
    }

    if let number = value as? NSNumber, CFGetTypeID(number) == CFBooleanGetTypeID() {
        return number.boolValue ? "true" : "false"
    }

    if PropertyListSerialization.propertyList(value, isValidFor: .xml) {
        do {
            let data = try PropertyListSerialization.data(
                fromPropertyList: value,
                format: .xml,
                options: 0
            )
            return String(decoding: data, as: UTF8.self)
        } catch {
            return String(describing: value)
        }
    }

    return String(describing: value)
}

func jsonString(from object: Any) -> String {
    guard JSONSerialization.isValidJSONObject(object) else {
        return String(describing: object)
    }

    do {
        let data = try JSONSerialization.data(withJSONObject: object, options: [.prettyPrinted, .sortedKeys])
        return String(decoding: data, as: UTF8.self)
    } catch {
        return String(describing: object)
    }
}

@_cdecl("sc_handle_retain")
public func sc_handle_retain(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    return Unmanaged<AnyObject>.fromOpaque(raw).retain().toOpaque()
}

@_cdecl("sc_handle_release")
public func sc_handle_release(_ raw: UnsafeMutableRawPointer?) {
    guard let raw else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(raw).release()
}

@_cdecl("sc_last_error_code")
public func sc_last_error_code() -> Int32 {
    Int32(SCError())
}

@_cdecl("sc_last_error_message")
public func sc_last_error_message() -> UnsafeMutableRawPointer? {
    let code = SCError()
    let raw = SCErrorString(code)
    return boxString(String(cString: raw))
}

@_cdecl("sc_string_len")
public func sc_string_len(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let box: StringBox = unretained(raw) else {
        return 0
    }
    return box.value.utf8.count
}

@_cdecl("sc_string_copy_utf8")
public func sc_string_copy_utf8(
    _ raw: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UInt8>?,
    _ capacity: Int
) -> Int {
    guard let box: StringBox = unretained(raw), let buffer, capacity > 0 else {
        return 0
    }

    let bytes = Array(box.value.utf8)
    let count = min(bytes.count, capacity)
    bytes.withUnsafeBufferPointer { source in
        if let baseAddress = source.baseAddress {
            buffer.update(from: baseAddress, count: count)
        }
    }
    return count
}

@_cdecl("sc_array_count")
public func sc_array_count(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let box: ArrayBox = unretained(raw) else {
        return 0
    }
    return box.values.count
}

@_cdecl("sc_array_get")
public func sc_array_get(_ raw: UnsafeMutableRawPointer?, _ index: Int) -> UnsafeMutableRawPointer? {
    guard let box: ArrayBox = unretained(raw), index >= 0, index < box.values.count else {
        return nil
    }
    return retain(box.values[index])
}

@_cdecl("sc_property_list_from_string")
public func sc_property_list_from_string(_ value: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    boxPropertyList(decodeCString(value))
}

@_cdecl("sc_property_list_from_bool")
public func sc_property_list_from_bool(_ value: UInt8) -> UnsafeMutableRawPointer? {
    boxPropertyList(value != 0)
}

@_cdecl("sc_property_list_from_json")
public func sc_property_list_from_json(_ value: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let json = decodeCString(value) else {
        return nil
    }
    return boxPropertyList(propertyListFromJSON(json))
}

@_cdecl("sc_property_list_copy_description")
public func sc_property_list_copy_description(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box: PropertyListBox = unretained(raw) else {
        return nil
    }
    return boxString(propertyListDescription(box.value))
}
