import Foundation
import SystemConfiguration

final class ConsoleUserBox {
    let name: String
    let uid: UInt32
    let gid: UInt32

    init(name: String, uid: uid_t, gid: gid_t) {
        self.name = name
        self.uid = UInt32(uid)
        self.gid = UInt32(gid)
    }
}

private func consoleUser(_ raw: UnsafeMutableRawPointer?) -> ConsoleUserBox? {
    unretained(raw)
}

@_cdecl("sc_console_user_copy_current")
public func sc_console_user_copy_current() -> UnsafeMutableRawPointer? {
    var uid: uid_t = 0
    var gid: gid_t = 0
    guard let name = SCDynamicStoreCopyConsoleUser(nil, &uid, &gid) as String? else {
        return nil
    }
    return retain(ConsoleUserBox(name: name, uid: uid, gid: gid))
}

@_cdecl("sc_console_user_copy_name")
public func sc_console_user_copy_name(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let box = consoleUser(raw) else {
        return nil
    }
    return boxString(box.name)
}

@_cdecl("sc_console_user_get_uid")
public func sc_console_user_get_uid(_ raw: UnsafeMutableRawPointer?) -> UInt32 {
    guard let box = consoleUser(raw) else {
        return 0
    }
    return box.uid
}

@_cdecl("sc_console_user_get_gid")
public func sc_console_user_get_gid(_ raw: UnsafeMutableRawPointer?) -> UInt32 {
    guard let box = consoleUser(raw) else {
        return 0
    }
    return box.gid
}
