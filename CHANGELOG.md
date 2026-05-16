# Changelog

## [0.1.0] - 2026-05-16

### Added

- `Reachability` wrappers for `SCNetworkReachabilityCreateWithName`,
  `SCNetworkReachabilityCreateWithAddress`, `GetFlags`, `SetCallback`, and
  `ScheduleWithRunLoop`.
- `DynamicStore` wrappers for `SCDynamicStoreCreate`, `CopyValue`, `SetValue`,
  `CopyKeyList`, and `SetNotificationKeys`.
- `NetworkInterface` wrappers for `SCNetworkInterfaceCopyAll` and key metadata
  accessors (`BSDName`, interface type, display name, hardware address).
- `Preferences` and `NetworkService` wrappers for `SCPreferencesCreate`,
  `Lock`, `SetValue`, `CommitChanges`, `SCNetworkServiceCopyAll`,
  `SCNetworkServiceGetServiceID`, and `SCNetworkServiceGetEnabled`.
- `examples/01_interfaces_and_reachability.rs` smoke example covering network
  interface enumeration and an `apple.com` reachability query.
