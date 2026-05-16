import Foundation
import SystemConfiguration

@_cdecl("sc_copy_last_error_json")
public func sc_copy_last_error_json() -> UnsafeMutableRawPointer? {
    let error = SCCopyLastError()
    var payload: [String: Any] = [
        "domain": CFErrorGetDomain(error) as String,
        "code": CFErrorGetCode(error),
        "description": CFErrorCopyDescription(error) as String,
    ]
    if let failureReason = CFErrorCopyFailureReason(error) as String? {
        payload["failure_reason"] = failureReason
    }
    if let recoverySuggestion = CFErrorCopyRecoverySuggestion(error) as String? {
        payload["recovery_suggestion"] = recoverySuggestion
    }
    return boxString(jsonString(from: payload))
}

@_cdecl("sc_system_configuration_error_domain")
public func sc_system_configuration_error_domain() -> UnsafeMutableRawPointer? {
    boxString(kCFErrorDomainSystemConfiguration as String)
}
