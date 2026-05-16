# Changelog

## [0.2.1] - 2026-05-16

### Added

- `NetworkConnection`, `BondInterface`, `BondStatus`, `VlanInterface`, and
  `SystemConfiguration` safe wrappers for the remaining public macOS families in
  the audited headers.
- Advanced `DynamicStore` helpers for callbacks, run-loop sources, dispatch
  queues, bulk updates, DHCP accessors, and formatted key creation.
- Advanced `Preferences` helpers for authorization, callbacks, run-loop
  scheduling, dispatch queues, and type IDs.
- Full `SCSchemaDefinitions.h` exposure through `SchemaCatalog::all` plus
  symbol-name lookup helpers.

### Changed

- Closed the final 259 non-exempt coverage gaps in `COVERAGE_AUDIT.md`, taking
  the audited bridge coverage to 100.00%.
- Expanded smoke tests and the schema example to validate the new coverage.

## [0.2.0] - 2026-05-16

### Changed

- Replaced the old direct-first C FFI architecture with a SwiftPM-built bridge
  patterned after `screencapturekit-rs`.
- Moved the primary macOS implementation behind per-area Swift bridge files plus
  matching safe Rust modules.
- Kept direct C symbols available behind the `raw-ffi` Cargo feature.

### Added

- Safe wrappers for the requested logical areas: `DynamicStore`,
  `NetworkConfiguration`, `Reachability`, `Preferences`, `Schema`,
  `NetworkService`, `NetworkSet`, `NetworkInterface`, `NetworkProtocol`,
  `ConsoleUser`, and `CaptiveNetwork`.
- 11 numbered examples covering each logical area.
- 11 per-area smoke test files and shared test helpers.
- `COVERAGE.md` with a per-header audit of implemented, partial, and skipped
  APIs.

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
