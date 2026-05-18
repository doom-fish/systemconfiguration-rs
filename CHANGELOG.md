# Changelog

## [0.5.2] - 2026-05-18

### Changed

- Re-exported `Boolean` from `apple_cf::raw` in `raw_ffi`, removing the remaining crate-local primitive alias.

## [0.5.1] - 2026-05-18

- Widen apple-cf version bound to `<0.10` so 0.9.x resolves.

## [0.5.0] - 2026-05-18

### Changed

- Re-exported the raw `CFAllocatorRef`, `CFArrayRef`, `CFDataRef`, `CFDateRef`,
  `CFDictionaryRef`, `CFPropertyListRef`, `CFRunLoopRef`, and `CFStringRef`
  typedefs from `apple-cf` in `raw_ffi`, removing the crate-local duplicates.
- Added `apple-cf` (`>=0.8, <0.9`) with `default-features = false` and the
  `cg` feature enabled for shared CoreFoundation/CoreGraphics type coverage.
- Bumped the crate version to `0.5.0`.

## [0.4.1] - 2026-05-18

### Fixed

- **Thread join in `DynamicStoreNotificationStream`**: `sc_dynamic_store_notification_unsubscribe`
  now waits for the run-loop thread to exit (`threadDone.wait()`) before returning.
  Previously, `CFRunLoopStop`+`CFRunLoopWakeUp` were called but the thread was
  never joined, creating a window where the thread could access the Rust sender
  box after `Drop` freed it.  A `DispatchSemaphore` (`threadDone`) is now
  signalled by the thread closure immediately after `CFRunLoopRun()` returns.
- **Panic safety in `extern "C"` callbacks**: all three `async_api` callbacks
  (`dynamic_store_async_cb`, `reachability_async_cb`, `preferences_async_cb`)
  now wrap user-code invocations in
  `doom_fish_utils::panic_safe::catch_user_panic`.  The synchronous
  `dynamic_store_callback` in `dynamic_store.rs` now wraps user closures in
  `std::panic::catch_unwind`.  Unhandled panics across the Swift/C FFI boundary
  are undefined behaviour.
- **SAFETY comments** added to the three `ctx`-pointer dereferences in `async_api.rs`
  callbacks and to the `info`-pointer dereference in `dynamic_store_callback`.
- **`unsafe impl Send/Sync for SubscriptionHandle`** now carries doc comments
  explaining the single-use / no-alias ownership invariants that make them sound.



### Added

- `async` feature with an `async_api` module exposing three
  `BoundedAsyncStream`-based notification streams:
  - `DynamicStoreNotificationStream` — wraps `SCDynamicStore` key-change
    notifications via a dedicated `CFRunLoop` thread.
  - `ReachabilityStream` — wraps `SCNetworkReachability` callbacks via a
    private dispatch queue.
  - `PreferencesNotificationStream` — wraps `SCPreferences` notifications via a
    private dispatch queue.
- Swift bridge entry points in `AsyncStream.swift` for async subscribe /
  unsubscribe lifecycle management.
- Async examples `50_async_dynamic_store`, `51_async_reachability`, and
  `52_async_preferences`.
- Async smoke coverage in `tests/async_stream_tests.rs`.

### Changed

- Bumped the crate version to `0.4.0`.

## [0.3.0] - 2026-05-17

### Added

- Public `schema_definitions` module with compile-time SystemConfiguration schema,
  interface, protocol, status, bond, reachability, captive-network, and
  connection constants exposed as Rust `pub const`s.
- `examples/23_schema_definitions_constants.rs` and
  `tests/schema_definitions_tests.rs` smoke coverage for the new constants.

### Changed

- Reclassified every stale entry in `COVERAGE_AUDIT_V2.md`, closing all 475 v2
  audit gaps and bringing the v2 audit to 582 verified symbols with 13
  documented exemptions.
- Verified the existing safe-wrapper coverage for `BondInterface`,
  `VlanInterface`, `NetworkConnection`, `DynamicStore`, `NetworkReachability`,
  `NetworkService`, `NetworkSet`, `Preferences`, `NetworkInterface`,
  `NetworkProtocol`, and `SystemConfiguration`.
- Bumped the crate version to `0.3.0`.

## [0.2.2] - 2026-05-17

### Added

- `DynamicStore::new_with_options` and
  `DynamicStore::new_with_options_and_callback` for the remaining generic
  `SCDynamicStoreCreateWithOptions` coverage.
- `Reachability::type_id`, `Reachability::set_callback_send`,
  `Reachability::set_dispatch_queue_global`, and
  `Reachability::clear_dispatch_queue` to close the last reachability gaps.

### Changed

- Closed the remaining safe-wrapper partials in `COVERAGE.md`, bringing the
  non-skipped safe audit to 100%.
- Bumped the crate version to `0.2.2`.

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
